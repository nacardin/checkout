#!/bin/bash
source .env
curl -s -X POST https://api.sandbox.checkout.com/payment-sessions \
  -H "Authorization: Bearer $CKO_SECRET_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 2000,
    "currency": "USD",
    "processing_channel_id": "'$CKO_PROCESSING_CHANNEL_ID'",
    "reference": "test",
    "success_url": "http://localhost/success",
    "failure_url": "http://localhost/failure"
  }'
