CREATE SCHEMA IF NOT EXISTS chat;

CREATE TABLE chat.user (
    id SERIAL PRIMARY KEY,
    alias VARCHAR(50) NOT NULL UNIQUE,  
    email VARCHAR(255) UNIQUE,         
    password_hash TEXT NOT NULL,        
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_login_at TIMESTAMP WITH TIME ZONE,
    status text DEFAULT 'offline' NOT NULL,  
    is_active BOOLEAN DEFAULT false NOT NULL               
);

CREATE TABLE chat.user_ip (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES chat.user(id) ON DELETE CASCADE, 
    ip_address INET NOT NULL,
    first_seen_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    last_seen_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL, 
    UNIQUE(user_id, ip_address) 
);


CREATE TABLE chat.message (
    id BIGSERIAL PRIMARY KEY,          
    sender_id INTEGER NOT NULL REFERENCES chat.user(id) ON DELETE RESTRICT,
    receiver_id INTEGER NOT NULL REFERENCES chat.user(id) ON DELETE RESTRICT,
    content TEXT NOT NULL,
    content_type VARCHAR(50) DEFAULT 'text' NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    delivered_at TIMESTAMP WITH TIME ZONE,  
    read_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    status TEXT DEFAULT 'sent' NOT NULL, 
    is_encrypted BOOLEAN DEFAULT false NOT NULL  
);


CREATE INDEX idx_message_sender ON chat.message(sender_id);
CREATE INDEX idx_message_receiver ON chat.message(receiver_id);
CREATE INDEX idx_message_created_at ON chat.message(created_at);
CREATE INDEX idx_user_status ON chat.user(status);
CREATE INDEX idx_user_alias ON chat.user(alias);
CREATE INDEX idx_user_ip_address ON chat.user_ip(ip_address);


