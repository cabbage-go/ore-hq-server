CREATE TABLE submissions (
  id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
  miner_id INT NOT NULL,
  challenge_id INT NOT NULL,
  difficulty TINYINT NOT NULL,
  nonce BIGINT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
)
