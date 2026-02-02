-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Categories table
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Products table
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    specifications JSONB DEFAULT '{}',
    image_url VARCHAR(500),
    additional_images JSONB DEFAULT '[]',
    regulatory_info JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Quotes table
CREATE TABLE quotes (
    id SERIAL PRIMARY KEY,
    company_name VARCHAR(255) NOT NULL,
    company_tax_id VARCHAR(50),
    contact_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    product_ids INTEGER[] NOT NULL,
    estimated_quantity TEXT,
    message TEXT,
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMP DEFAULT NOW(),
    contacted_at TIMESTAMP,
    notes TEXT
);

-- Admins table
CREATE TABLE admins (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    created_at TIMESTAMP DEFAULT NOW(),
    last_login TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_active ON products(is_active);
CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_quotes_status ON quotes(status);
CREATE INDEX idx_quotes_created ON quotes(created_at DESC);

-- Full-text search index for products (Spanish language)
CREATE INDEX idx_products_search ON products 
USING gin(to_tsvector('spanish', name || ' ' || COALESCE(description, '')));

-- Insert default admin user (password: admin123)
-- Password hash for 'admin123' using bcrypt
INSERT INTO admins (email, password_hash, name) 
VALUES (
    'admin@labmedical.com', 
    '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzpLaEg7Pu',
    'Administrator'
);

-- Insert sample categories
INSERT INTO categories (name, slug, description) VALUES
('Equipos Médicos', 'equipos-medicos', 'Equipos y dispositivos médicos profesionales'),
('Instrumental Quirúrgico', 'instrumental-quirurgico', 'Instrumentos para procedimientos quirúrgicos'),
('Consumibles', 'consumibles', 'Productos de uso médico desechables'),
('Mobiliario Clínico', 'mobiliario-clinico', 'Mobiliario y equipamiento para clínicas y hospitales');
