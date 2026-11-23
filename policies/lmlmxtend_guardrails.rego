package main

# ==============================================================================
# LMLMxtend Architectural Guardrails (Evaluates modules/lmlmxtend-runtime)
# ==============================================================================

# 1. Block Public IP Exposure on LMLMxtend Runtime Nodes
violations[msg] {
    resource := input.resource_changes[_]
    resource.type == "aws_instance"
    contains(resource.address, "lmlmxtend")
    
    resource.change.after.associate_public_ip_address == true
    msg := sprintf("❌ LMLMxtend VIOLATION: Node '%v' cannot have a public IP. LMLM requires local-first private execution.", [resource.address])
}

# 2. Enforce Loopback-Only Boundary on Processor Bridge Security Groups
violations[msg] {
    resource := input.resource_changes[_]
    resource.type == "aws_security_group"
    contains(resource.address, "lmlmxtend")
    
    ingress_rule := resource.change.after.ingress[_]
    ingress_rule.cidr_blocks[_] == "0.0.0.0/0"
    msg := sprintf("❌ LMLMxtend VIOLATION: Security group '%v' exposes the LMLM Processor v1 bridge to 0.0.0.0/0. Loopback/private VPC only.", [resource.address])
}

# 3. Require Encrypted Storage for .onnx Models & Agent State
violations[msg] {
    resource := input.resource_changes[_]
    resource.type == "aws_instance"
    contains(resource.address, "lmlmxtend")
    
    block := resource.change.after.root_block_device[_]
    block.encrypted != true
    msg := sprintf("❌ LMLMxtend VIOLATION: Root volume on '%v' must be encrypted to protect local ONNX weights and agent mailboxes.", [resource.address])
}

# 4. Enforce 8 MiB (8,388,608 bytes) Goal-Planner Payload Cap
violations[msg] {
    resource := input.resource_changes[_]
    contains(resource.address, "lmlmxtend")
    
    # Inspects user_data or environment variables if max payload size is overridden
    user_data := resource.change.after.user_data
    contains(user_data, "LMLM_MAX_GOAL_BYTES=")
    
    # Extract numeric value if configured above 8 MiB
    regex.match("LMLM_MAX_GOAL_BYTES=(838860[9]|83886[1-9][0-9]|8388[7-9][0-9]{2}|8389[0-9]{3}|839[0-9]{4}|8[4-9][0-9]{5}|9[0-9]{6}|[1-9][0-9]{7,})", user_data)
    msg := sprintf("❌ LMLMxtend VIOLATION: '%v' exceeds the strict 8 MiB (8388608 bytes) execute_goal aggregate payload cap.", [resource.address])
}
