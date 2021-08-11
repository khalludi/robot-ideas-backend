-- Create users and ideas

CREATE TABLE users_table (
    id INT(11) NOT NULL AUTO_INCREMENT,
    username VARCHAR(30) NOT NULL,
    PRIMARY KEY(id)
);

CREATE TABLE ideas (
    id INT(11) NOT NULL AUTO_INCREMENT,
    userId INT(11) NOT NULL,
    body VARCHAR(1000) NOT NULL,
    likes INT NOT NULL DEFAULT 0,
    dislikes INT NOT NULL DEFAULT 0,
    PRIMARY KEY(id),
    FOREIGN KEY (userId) REFERENCES Users_Table(id)
);
