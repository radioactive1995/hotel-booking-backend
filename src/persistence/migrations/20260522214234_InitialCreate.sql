CREATE TABLE hotels (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

   name VARCHAR(255) NOT NULL,
   description TEXT,

   address TEXT NOT NULL,
   city VARCHAR(100) NOT NULL,
   country VARCHAR(100) NOT NULL,

   rating INTEGER CHECK (rating >= 0 AND rating <= 5),

   check_in_time TIME NOT NULL,
   check_out_time TIME NOT NULL
);