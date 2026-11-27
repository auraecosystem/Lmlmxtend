data "aws_region" "current" {}

locals {
  interface_services = toset(concat(
    ["logs", "ec2", "ecs", "lambda"],
    var.runtime_type == "ec2" ? ["ssm", "ssmmessages", "ec2messages"] : [],
    var.runtime_type == "ecs" ? ["ecr.api", "ecr.dkr"] : []
  ))
}

# Security Group allowing HTTPS from the runtime security group to VPC Endpoints
resource "aws_security_group" "vpce_sg" {
  count       = var.create_vpc_endpoints ? 1 : 0
  name        = "${local.name_prefix}-vpce-sg"
  description = "Allow HTTPS traffic from runtime to AWS VPC Interface Endpoints"
  vpc_id      = var.vpc_id

  ingress {
    description     = "HTTPS from runtime SG"
    from_port       = 443
    to_port         = 443
    protocol        = "tcp"
    security_groups = [aws_security_group.runtime_sg.id]
  }

  tags = local.common_tags
}

# Interface Endpoints (SSM, ECR, CloudWatch Logs, EC2, ECS, Lambda)
resource "aws_vpc_endpoint" "interface" {
  for_each            = var.create_vpc_endpoints ? local.interface_services : toset([])
  vpc_id              = var.vpc_id
  service_name        = "com.amazonaws.${data.aws_region.current.name}.${each.key}"
  vpc_endpoint_type   = "Interface"
  subnet_ids          = [var.subnet_id]
  security_group_ids  = [aws_security_group.vpce_sg[0].id]
  private_dns_enabled = true

  tags = merge(local.common_tags, {
    Name = "${local.name_prefix}-vpce-${each.key}"
  })
}

# S3 Gateway Endpoint (Free; required for ECR container image pulls and SSM agents)
resource "aws_vpc_endpoint" "s3" {
  count             = var.create_vpc_endpoints && length(var.route_table_ids) > 0 ? 1 : 0
  vpc_id            = var.vpc_id
  service_name      = "com.amazonaws.${data.aws_region.current.name}.s3"
  vpc_endpoint_type = "Gateway"
  route_table_ids   = var.route_table_ids

  tags = merge(local.common_tags, {
    Name = "${local.name_prefix}-vpce-s3"
  })
}
