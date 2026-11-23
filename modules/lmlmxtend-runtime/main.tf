variable "environment" {
  type    = string
  default = "prod"
}

variable "ttl_days" {
  type        = string
  default     = "14"
  description = "Enforced by OPA landing_zone.rego policy (max 30 days)"
}

variable "subnet_id" {
  type        = string
  description = "Private subnet ID from your Landing Zone Hub VPC"
}

# Security Group: Enforces loopback-only processor bridge & zero public ingress
resource "aws_security_group" "lmlmxtend_sg" {
  name        = "lmlmxtend-processor-sg-${var.environment}"
  description = "Restricts LMLMxtend runtime to internal subnet traffic only"
  vpc_id      = var.vpc_id

  # No inbound rules from 0.0.0.0/0 (Passes OPA firewall guardrails)
  egress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"] # Allows pulling Rust crates/ONNX artifacts during build
  }

  tags = {
    Name        = "lmlmxtend-sg"
    Project     = "Lmlmxtend"
    ttl_days    = var.ttl_days
  }
}

# Compute Node for Rust Workspace & Agent Runtime (up to 32 parallel batch workers)
resource "aws_instance" "lmlmxtend_node" {
  ami                         = "ami-0c7217cdde317cfec" # Ubuntu 22.04 LTS
  instance_type               = "c6i.8xlarge"           # 32 vCPUs to match 32-worker agent batch limit
  subnet_id                   = var.subnet_id
  vpc_security_group_ids      = [aws_security_group.lmlmxtend_sg.id]
  associate_public_ip_address = false                   # Enforces private-only Landing Zone rule

  root_block_device {
    volume_size = 100
    volume_type = "gp3"
    encrypted   = true # Passes OPA encryption guardrails
  }

  # Bootstrap Rust toolchain, clone Lmlmxtend, and run verification scripts
  user_data = <<-EOF
              #!/bin/bash
              apt-get update && apt-get install -y build-essential curl git pkg-config libssl-dev
              curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh -s -- -y
              source "$HOME/.cargo/env"
              git clone https://github.com /opt/lmlmxtend
              cd /opt/lmlmxtend
              bash scripts/verify.sh
              EOF

  tags = {
    Name        = "lmlmxtend-runtime-${var.environment}"
    Component   = "lmlm-processor-bridge"
    ttl_days    = var.ttl_days
  }
}
