from datetime import datetime, timedelta, timezone
from unittest.mock import MagicMock, patch
import pytest

import index as reaper


def iso_offset(hours: int) -> str:
    return (datetime.now(timezone.utc) + timedelta(hours=hours)).isoformat()


def test_is_expired_checks_module_env_and_timestamp():
    past = iso_offset(-2)
    future = iso_offset(24)

    # Valid expired resource in target environment
    assert reaper.is_expired({
        "Module": "lmlmxtend-runtime",
        "Environment": "prod",
        "ExpiresAt": past,
    }) is True

    # Active resource (not yet expired)
    assert reaper.is_expired({
        "Module": "lmlmxtend-runtime",
        "Environment": "prod",
        "ExpiresAt": future,
    }) is False

    # Wrong environment must be ignored
    assert reaper.is_expired({
        "Module": "lmlmxtend-runtime",
        "Environment": "dev",
        "ExpiresAt": past,
    }) is False

    # Unrelated module must be ignored
    assert reaper.is_expired({
        "Module": "other-module",
        "Environment": "prod",
        "ExpiresAt": past,
    }) is False


@patch("index.boto3.client")
def test_reap_ec2_terminates_only_expired_instances(mock_boto_client):
    mock_ec2 = MagicMock()
    mock_boto_client.return_value = mock_ec2

    mock_ec2.describe_instances.return_value = {
        "Reservations": [
            {
                "Instances": [
                    {
                        "InstanceId": "i-expired123",
                        "Tags": [
                            {"Key": "Module", "Value": "lmlmxtend-runtime"},
                            {"Key": "Environment", "Value": "prod"},
                            {"Key": "ExpiresAt", "Value": iso_offset(-1)},
                        ],
                    },
                    {
                        "InstanceId": "i-active456",
                        "Tags": [
                            {"Key": "Module", "Value": "lmlmxtend-runtime"},
                            {"Key": "Environment", "Value": "prod"},
                            {"Key": "ExpiresAt", "Value": iso_offset(48)},
                        ],
                    },
                ]
            }
        ]
    }

    reaper.reap_ec2()

    mock_ec2.terminate_instances.assert_called_once_with(
        InstanceIds=["i-expired123"]
    )


@patch("index.boto3.client")
def test_reap_ecs_scales_down_and_deletes_expired_service(mock_boto_client):
    mock_ecs = MagicMock()
    mock_boto_client.return_value = mock_ecs

    cluster_arn = "arn:aws:ecs:us-east-1:123456789012:cluster/test-cluster"
    svc_arn = "arn:aws:ecs:us-east-1:123456789012:service/test-cluster/expired-svc"

    mock_ecs.list_clusters.return_value = {"clusterArns": [cluster_arn]}
    mock_ecs.list_services.return_value = {"serviceArns": [svc_arn]}
    mock_ecs.describe_services.return_value = {
        "services": [
            {
                "serviceName": "expired-svc",
                "tags": [
                    {"key": "Module", "value": "lmlmxtend-runtime"},
                    {"key": "Environment", "value": "prod"},
                    {"key": "ExpiresAt", "value": iso_offset(-5)},
                ],
            }
        ]
    }

    reaper.reap_ecs()

    mock_ecs.update_service.assert_called_once_with(
        cluster=cluster_arn, service="expired-svc", desiredCount=0
    )
    mock_ecs.delete_service.assert_called_once_with(
        cluster=cluster_arn, service="expired-svc", force=True
    )


@patch("index.boto3.client")
def test_reap_lambda_skips_reaper_itself_and_deletes_expired(mock_boto_client):
    mock_lam = MagicMock()
    mock_boto_client.return_value = mock_lam

    mock_lam.list_functions.return_value = {
        "Functions": [
            {
                "FunctionName": "lmlmxtend-runtime-prod-ttl-reaper",
                "FunctionArn": "arn:aws:lambda:us-east-1:123456789012:function:lmlmxtend-runtime-prod-ttl-reaper",
            },
            {
                "FunctionName": "lmlmxtend-runtime-prod",
                "FunctionArn": "arn:aws:lambda:us-east-1:123456789012:function:lmlmxtend-runtime-prod",
            },
        ]
    }
    mock_lam.list_tags.return_value = {
        "Tags": {
            "Module": "lmlmxtend-runtime",
            "Environment": "prod",
            "ExpiresAt": iso_offset(-3),
        }
    }

    reaper.reap_lambda()

    mock_lam.delete_function.assert_called_once_with(
        FunctionName="lmlmxtend-runtime-prod"
    )
