CREATE TABLE room_types (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    hotel_id INTEGER NOT NULL,

    name VARCHAR(255) NOT NULL,
    description TEXT,
    capacity INTEGER NOT NULL CHECK (capacity > 0),
    base_price NUMERIC(10, 2) NOT NULL CHECK (base_price > 0),
    total_rooms INTEGER NOT NULL CHECK (total_rooms > 0),

    CONSTRAINT fk_room_types_hotel
        FOREIGN KEY (hotel_id)
            REFERENCES hotels(id)
            ON DELETE CASCADE);