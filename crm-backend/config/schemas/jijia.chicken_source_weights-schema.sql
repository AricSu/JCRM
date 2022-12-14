/*!40101 SET NAMES binary*/;
CREATE TABLE `chicken_source_weights` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `weight` decimal(8,4) DEFAULT NULL,
  `order_id` varchar(45) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=52 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
