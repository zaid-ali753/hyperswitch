-- Your SQL goes here
CREATE TABLE google_pay_credentials (
    id SERIAL PRIMARY KEY,
    merchant_id VARCHAR(255) NOT NULL,
    connector_name VARCHAR(32) NOT NULL,
    allowed_payment_methods TEXT NOT NULL DEFAULT '[]',
    allowed_auth TEXT NOT NULL DEFAULT '[]',
    credentials JSON
);

CREATE UNIQUE INDEX google_pay_credentials_index ON google_pay_credentials (merchant_id, connector_name);