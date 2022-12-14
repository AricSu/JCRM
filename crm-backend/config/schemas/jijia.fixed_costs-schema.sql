/*!40101 SET NAMES binary*/;
CREATE TABLE `fixed_costs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `cost_name` varchar(400) DEFAULT NULL,
  `cost_price` decimal(8,2) DEFAULT NULL,
  `comments` varchar(800) DEFAULT NULL,
  `cost_date` timestamp NULL DEFAULT NULL,
  `status` int DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
