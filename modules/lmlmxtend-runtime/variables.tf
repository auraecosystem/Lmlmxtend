variable "environment" {
  description = "Deployment environment name (e.g., dev, staging, prod)."
  type        = string

  validation {
    condition     = contains(["dev", "staging", "prod"], var.environment)
    error_message = "Environment must be one of: dev, staging, or prod."
  }
}

variable "vpc_id" {
  description = "ID of the target AWS VPC where the runtime will be deployed."
  type        = string
}

variable "subnet_id" {
  description = "ID of the private subnet for the runtime network interface."
  type        = string
}

variable "ttl_days" {
  description = "Number of days before the runtime resources expire."
  type        = number
  default     = 14

  validation {
    condition     = var.ttl_days >= 1 && var.ttl_days <= 365
    error_message = "ttl_days must be between 1 and 365."
  }
}

variable "instance_type" {
  description = "EC2 instance type for the runtime node."
  type        = string
  default     = "t3.medium"
}
