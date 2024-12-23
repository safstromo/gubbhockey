-- Players Table
CREATE TABLE IF NOT EXISTS Player (
    player_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    access_group VARCHAR(20)
);

-- Gamedays Table
CREATE TABLE IF NOT EXISTS Gameday (
    gameday_id SERIAL PRIMARY KEY,
    date DATE NOT NULL
);

-- Join Table for Player-Gameday relationship
CREATE TABLE IF NOT EXISTS Player_Gameday (
    player_id INT,
    gameday_id INT,
    PRIMARY KEY (player_id, gameday_id),
    FOREIGN KEY (player_id) REFERENCES Player(player_id) ON DELETE CASCADE,
    FOREIGN KEY (gameday_id) REFERENCES Gameday(gameday_id) ON DELETE CASCADE
);
