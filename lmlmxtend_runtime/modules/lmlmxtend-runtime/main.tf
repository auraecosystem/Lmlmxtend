terraform {
  required_version = ">= 1.5.0"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = ">= 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

variable "aws_region" {
  description = "AWS region for the hub VPC and runtime."
  type        = string
  default     = "us-east-1"
}

# Hub VPC with DNS hostnames enabled (required for PrivateLink VPC endpoints)
resource "aws_vpc" "aws_hub_vpc" {
  cidr_block           = "10.100.0.0/16"
  enable_dns_support   = true
  enable_dns_hostnames = true

  tags = {
    Name        = "aws-hub-vpc"
    Environment = "prod"
    ManagedBy   = "Terraform"
  }
}

# Isolated Private Subnet
resource "aws_subnet" "private_subnet" {
  vpc_id                  = aws_vpc.aws_hub_vpc.id
  cidr_block              = "10.100.1.0/24"
  map_public_ip_on_launch = false

  tags = {
    Name        = "aws-hub-private-subnet"
    Environment = "prod"
    ManagedBy   = "Terraform"
  }
}

# Private Route Table (no 0.0.0.0/0 internet route)
resource "aws_route_table" "private" {
  vpc_id = aws_vpc.aws_hub_vpc.id

  tags = {
    Name        = "aws-hub-private-rt"
    Environment = "prod"
    ManagedBy   = "Terraform"
  }
}

resource "aws_route_table_association" "private" {
  subnet_id      = aws_subnet.private_subnet.id
  route_table_id = aws_route_table.private.id
}

# LMLMxtend Runtime Module
module "lmlmxtend_runtime" {
  source               = "./modules/lmlmxtend-runtime"
  environment          = "prod"
  vpc_id               = aws_vpc.aws_hub_vpc.id
  subnet_id            = aws_subnet.private_subnet.id
  route_table_ids      = [aws_route_table.private.id]
  ttl_days             = 14
  runtime_type         = "ec2"
  create_vpc_endpoints = true
}

output "runtime_id" {
  description = "Provisioned compute resource identifier."
  value       = module.lmlmxtend_runtime.runtime_id
}

output "runtime_expires_at" {
  description = "UTC timestamp when the runtime expires."
  value       = module.lmlmxtend_runtime.expires_at
}
