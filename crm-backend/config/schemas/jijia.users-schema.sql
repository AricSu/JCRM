/*!40101 SET NAMES binary*/;
CREATE TABLE `users` (
  `user_id` bigint NOT NULL,
  `name` varchar(45) DEFAULT NULL,
  `password` varchar(45) DEFAULT NULL,
  `introduction` varchar(500) DEFAULT NULL,
  `avatar` varchar(500) DEFAULT NULL,
  `roles` varchar(45) DEFAULT NULL,
  PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
