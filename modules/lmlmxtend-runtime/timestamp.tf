locals {
  name_prefix = "lmlmxtend-runtime-${var.environment}"
  expires_at  = timeadd(timestamp(), "${tonumber(var.ttl_days) * 24}h")

  common_tags = {
    Name        = local.name_prefix
    Environment = var.environment
    Module      = "lmlmxtend-runtime"
    TTLDays     = tostring(var.ttl_days)
    ExpiresAt   = local.expires_at
    ManagedBy   = "Terraform"
  }
}

# Fetch latest Amazon Linux 2023 AMI
data "aws_ami" "al2023" {
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["al2023-ami-2023.*-x86_64"]
  }
}

# Security Group: Zero inbound rules; outbound HTTPS only for AWS APIs & updates
resource "aws_security_group" "runtime_sg" {
  name        = "${local.name_prefix}-sg"
  description = "Security group for ${local.name_prefix}"
  vpc_id      = var.vpc_id

  egress {
    description = "Allow outbound HTTPS traffic"
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = local.common_tags
}

# IAM Role for SSM Session Manager access (no SSH ports required)
resource "aws_iam_role" "runtime_role" {
  name = "${local.name_prefix}-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "://amazonaws.com"
        }
      }
    ]
  })

  tags = local.common_tags
}

resource "aws_iam_role_policy_attachment" "ssm_core" {
  role       = aws_iam_role.runtime_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_iam_instance_profile" "runtime_profile" {
  name = "${local.name_prefix}-profile"
  role = aws_iam_role.runtime_role.name
}

# Private EC2 Runtime Instance
resource "aws_instance" "runtime" {
  ami                    = data.aws_ami.al2023.id
  instance_type          = var.instance_type
  subnet_id              = var.subnet_id
  vpc_security_group_ids = [aws_security_group.runtime_sg.id]
  iam_instance_profile   = aws_iam_instance_profile.runtime_profile.name

  associate_public_ip_address = false

  metadata_options {
    http_endpoint = "enabled"
    http_tokens   = "required" # Enforce IMDSv2
  }

  root_block_device {
    volume_type = "gp3"
    volume_size = 30
    encrypted   = true
  }

  lifecycle {
    ignore_changes = [tags["ExpiresAt"]]
  }

  tags = local.common_tags
}
