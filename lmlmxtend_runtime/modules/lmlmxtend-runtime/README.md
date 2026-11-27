# `lmlmxtend-runtime` Terraform Module

Provisions an isolated AWS compute runtime (`ec2`, `ecs`, or `lambda`) inside a private subnet with optional AWS PrivateLink VPC endpoints and an automated hourly EventBridge + Lambda Time-To-Live (TTL) reaper.

## Architecture Overview

* **Compute Flexibility:** Supports Amazon Linux 2023 EC2 instances (with SSM Session Manager), AWS Fargate services, or VPC-attached Python 3.12 Lambda functions.
* **Zero-Ingress Networking:** Attaches a dedicated security group blocking all inbound traffic and permitting outbound HTTPS (`443`) only.
* **NAT-Free PrivateLink Support:** Optionally provisions Interface and Gateway VPC endpoints for SSM, ECR, CloudWatch Logs, and S3.
* **Automated TTL Enforcement:** Stamps an `ExpiresAt` ISO-8601 UTC timestamp tag onto compute resources and cleans them up hourly via a dedicated Python reaper Lambda.

## Usage

```hcl
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
```

## Requirements

| Name | Version |
| :--- | :--- |
| `terraform` | `>= 1.5.0` |
| `aws` | `>= 5.0` |
| `archive` | `>= 2.4` |

## Inputs

| Name | Description | Type | Default | Required |
| :--- | :--- | :--- | :--- | :--- |
| `environment` | Deployment tier (`dev`, `staging`, or `prod`). | `string` | n/a | yes |
| `vpc_id` | Target AWS VPC identifier. | `string` | n/a | yes |
| `subnet_id` | Target private subnet identifier. | `string` | n/a | yes |
| `ttl_days` | Retention period in days (`1` to `365`). | `number` | `14` | no |
| `runtime_type` | Compute backend (`ec2`, `ecs`, or `lambda`). | `string` | `"ec2"` | no |
| `instance_type` | EC2 instance size when `runtime_type = "ec2"`. | `string` | `"t3.medium"` | no |
| `container_image` | Container image URI when `runtime_type = "ecs"`. | `string` | `"public.ecr.aws/amazonlinux/amazonlinux:2023"` | no |
| `create_vpc_endpoints` | Create PrivateLink endpoints in the subnet. | `bool` | `true` | no |
| `route_table_ids` | Private route table IDs for the S3 Gateway Endpoint. | `list(string)` | `[]` | no |

## Outputs

| Name | Description |
| :--- | :--- |
| `runtime_type` | Active compute backend (`ec2`, `ecs`, or `lambda`). |
| `runtime_id` | Instance ID, ECS Service ID, or Lambda Function ARN. |
| `security_group_id` | ID of the runtime security group. |
| `reaper_lambda_arn` | ARN of the hourly TTL reaper Lambda function. |
| `expires_at` | Calculated UTC expiration timestamp. |
