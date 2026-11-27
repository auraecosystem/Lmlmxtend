mock_provider "aws" {
  mock_data "aws_ami" {
    defaults = {
      id = "ami-0123456789abcdef0"
    }
  }

  mock_data "aws_region" {
    defaults = {
      name = "us-east-1"
    }
  }
}

mock_provider "archive" {}

variables {
  environment = "prod"
  vpc_id      = "vpc-0a1b2c3d4e5f67890"
  subnet_id   = "subnet-0a1b2c3d4e5f67890"
  ttl_days    = 14
}

# 1. Verify default EC2 runtime plan and tagging
run "validates_default_ec2_runtime" {
  command = plan

  assert {
    condition     = aws_instance.runtime[0].associate_public_ip_address == false
    error_message = "EC2 runtime must not assign a public IP address."
  }

  assert {
    condition     = aws_instance.runtime[0].tags["Environment"] == "prod"
    error_message = "EC2 runtime must be tagged with the prod environment."
  }

  assert {
    condition     = aws_instance.runtime[0].tags["TTLDays"] == "14"
    error_message = "TTLDays tag must match the configured ttl_days input."
  }
}

# 2. Verify ECS Fargate backend selection
run "validates_ecs_runtime_selection" {
  command = plan

  variables {
    runtime_type = "ecs"
  }

  assert {
    condition     = length(aws_ecs_service.runtime) == 1
    error_message = "ECS service should be planned when runtime_type is ecs."
  }

  assert {
    condition     = length(aws_instance.runtime) == 0
    error_message = "EC2 instance must not be created when runtime_type is ecs."
  }
}

# 3. Verify Lambda backend selection
run "validates_lambda_runtime_selection" {
  command = plan

  variables {
    runtime_type = "lambda"
  }

  assert {
    condition     = length(aws_lambda_function.runtime) == 1
    error_message = "Lambda function should be planned when runtime_type is lambda."
  }
}

# 4. Verify input validation rejects invalid environments
run "rejects_invalid_environment" {
  command = plan

  variables {
    environment = "sandbox"
  }

  expect_failures = [
    var.environment
  ]
}

# 5. Verify input validation rejects out-of-range TTL values
run "rejects_invalid_ttl_days" {
  command = plan

  variables {
    ttl_days = 400
  }

  expect_failures = [
    var.ttl_days
  ]
}
