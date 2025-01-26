-- Players Table
CREATE TABLE IF NOT EXISTS Player (
    player_id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    given_name VARCHAR(50) NOT NULL,
    family_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    access_group VARCHAR(20) NOT NULL,
    is_goalkeeper bool DEFAULT FALSE NOT NULL

);

-- Gamedays Table
CREATE TABLE IF NOT EXISTS Gameday (
    gameday_id SERIAL PRIMARY KEY,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL
);

-- Join Table for Player-Gameday relationship
CREATE TABLE IF NOT EXISTS Player_Gameday (
    player_id INT NOT NULL,
    gameday_id INT NOT NULL,
    PRIMARY KEY (player_id, gameday_id),
    FOREIGN KEY (player_id) REFERENCES Player(player_id) ON DELETE CASCADE,
    FOREIGN KEY (gameday_id) REFERENCES Gameday(gameday_id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pkce_store (
    id SERIAL PRIMARY KEY, 
    csrf_token TEXT NOT NULL,                     -- CSRF token associated with the request
    pkce_verifier TEXT NOT NULL,                  -- PKCE verifier value
    created_at TIMESTAMPTZ DEFAULT NOW(),           -- Timestamp of creation
    expires_at TIMESTAMPTZ NOT NULL                 -- Expiration timestamp
);

DROP INDEX IF EXISTS idx_csrf_token;

-- Index to quickly look up by CSRF token
CREATE UNIQUE INDEX idx_csrf_token ON pkce_store (csrf_token);

CREATE TABLE IF NOT EXISTS Session (
    session_id UUID PRIMARY KEY,                  -- Unique session ID
    player_id INT NOT NULL,                       -- Associated player ID
    created_at TIMESTAMPTZ DEFAULT NOW(),         -- Creation timestamp
    expires_at TIMESTAMPTZ NOT NULL,              -- Expiration timestamp

    FOREIGN KEY (player_id) REFERENCES Player(player_id) ON DELETE CASCADE
);

-- Cup Table
CREATE TABLE IF NOT EXISTS Cup (
    cup_id SERIAL PRIMARY KEY,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    title VARCHAR(100) NOT NULL,
    info TEXT NOT NULL
);

-- Join Table for Player-Cup relationship
CREATE TABLE IF NOT EXISTS Player_Cup (
    player_id INT NOT NULL,
    cup_id INT NOT NULL,
    position VARCHAR(50) NOT NULL,
    PRIMARY KEY (player_id, cup_id),
    FOREIGN KEY (player_id) REFERENCES Player(player_id) ON DELETE CASCADE,
    FOREIGN KEY (cup_id) REFERENCES Cup(cup_id) ON DELETE CASCADE
);

