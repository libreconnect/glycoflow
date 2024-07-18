CREATE TABLE access_tokens (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id VARCHAR(255) NOT NULL,
  access_token BYTEA NOT NULL,
  nonce BYTEA NOT NULL,
  patient_id BYTEA NOT NULL,
  created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP
)