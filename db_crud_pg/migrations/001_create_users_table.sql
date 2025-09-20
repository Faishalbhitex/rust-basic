-- Create users table with sample data
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    age INTEGER NOT NULL,
    height NUMERIC(5,2) NOT NULL
);

-- Insert sample data
INSERT INTO users (name, age, height) VALUES 
('Faishal', 24, 161.00),
('Fahmi', 16, 169.00),
('Farham', 23, 165.00),
('Nasrul', 23, 170.00),
('Andi', 27, 178.00),
('Ahmad Saputra', 27, 170.00),
('Andrean', 27, 182.00),
('Fadhil', 24, 161.00),
('Jamal', 30, 160.00),
('Taufik', 24, 165.00),
('Ilham', 25, 160.00),
('Firman', 24, 165.00)
ON CONFLICT (id) DO NOTHING;
