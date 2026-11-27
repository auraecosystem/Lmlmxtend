@node Infrastructure Runtime
@chapter Infrastructure Runtime
@cindex Terraform module
@cindex lmlmxtend-runtime
@cindex TTL reaper

The @code{lmlmxtend-runtime} Terraform module provisions isolated compute workloads inside a private AWS subnet with automated Time-To-Live (TTL) lifecycle enforcement.

@menu
* Module Usage::              Invoking the Terraform module.
* Input Variables::           Required and optional configuration arguments.
* Compute Backends::          Choosing between EC2, ECS Fargate, and Lambda.
* Private Networking::        VPC endpoints and security group isolation.
* Automated TTL Reaper::      How expired resources are cleaned up.
@end menu

@node Module Usage
@section Module Usage

Declare the child module in your root Terraform configuration and pass your hub VPC and private subnet identifiers:

@example
module "lmlmxtend_runtime" @{
  source          = "./modules/lmlmxtend-runtime"
  environment     = "prod"
  vpc_id          = aws_vpc.aws_hub_vpc.id
  subnet_id       = aws_subnet.private_subnet.id
  route_table_ids = [aws_route_table.private.id]
  ttl_days        = 14
  runtime_type    = "ecs"
@}
@end example

@node Input Variables
@section Input Variables

@table @code
@item environment
@cindex @code{environment} variable
Target deployment tier. Must be @code{dev}, @code{staging}, or @code{prod}.

@item vpc_id
@cindex @code{vpc_id} variable
Identifier of the existing AWS VPC where security groups and endpoints are created.

@item subnet_id
@cindex @code{subnet_id} variable
Identifier of the isolated private subnet hosting the compute workload.

@item ttl_days
@cindex @code{ttl_days} variable
Retention window in days (between @code{1} and @code{365}). Defaults to @code{14}.

@item runtime_type
@cindex @code{runtime_type} variable
Selects the underlying compute engine: @code{ec2} (default), @code{ecs}, or @code{lambda}.

@item create_vpc_endpoints
@cindex @code{create_vpc_endpoints} variable
Boolean flag (default @code{true}) controlling whether AWS PrivateLink endpoints are provisioned in the subnet.
@end table

@node Compute Backends
@section Compute Backends

@itemize @bullet
@item @strong{EC2 (@code{ec2}):} Launches an encrypted @code{gp3} Amazon Linux 2023 instance with IMDSv2 enforced and AWS Systems Manager (SSM) enabled for keyless shell access.
@item @strong{ECS Fargate (@code{ecs}):} Provisions a serverless Fargate cluster and task definition running in @code{awsvpc} mode without public IP assignment.
@item @strong{Lambda (@code{lambda}):} Deploys a VPC-attached Python 3.12 function for event-driven execution.
@end itemize

@node Private Networking
@section Private Networking

All compute backends attach to a dedicated security group that denies all inbound traffic and restricts outbound traffic to port @code{443} (HTTPS). When @code{create_vpc_endpoints} is enabled, AWS PrivateLink endpoints allow SSM, ECR, and CloudWatch Logs traffic to traverse the AWS backbone without a NAT Gateway.

@node Automated TTL Reaper
@section Automated TTL Reaper

During provisioning, Terraform stamps an @code{ExpiresAt} UTC ISO-8601 tag onto every compute resource. An Amazon EventBridge rule triggers the @code{lmlmxtend-runtime-@var{env}-ttl-reaper} Lambda function every hour to scan for resources whose @code{ExpiresAt} timestamp has passed and terminates them automatically.
