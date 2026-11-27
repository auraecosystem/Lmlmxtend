output "instance_id" {
  description = "The ID of the provisioned runtime EC2 instance."
  value       = aws_instance.runtime.id
}

output "private_ip" {
  description = "The private IP address assigned to the runtime instance."
  value       = aws_instance.runtime.private_ip
}

output "security_group_id" {
  description = "The ID of the runtime security group."
  value       = aws_security_group.runtime_sg.id
}

output "expires_at" {
  description = "Calculated UTC timestamp when the runtime reaches its TTL."
  value       = local.expires_at
}
