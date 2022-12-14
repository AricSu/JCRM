/*!40101 SET NAMES binary*/;
CREATE TABLE `customer` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `customer_id` bigint NOT NULL,
  `customer_name` varchar(45) DEFAULT NULL,
  `customer_status` int DEFAULT NULL,
  `customer_location` varchar(500) DEFAULT NULL,
  `customer_phone` bigint DEFAULT NULL,
  `product_price` decimal(4,2) DEFAULT NULL,
  `try_bussiness_time` timestamp NULL DEFAULT NULL,
  `start_bussiness_time` timestamp NULL DEFAULT NULL,
  `finish_bussiness_time` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `customer_id_UNIQUE` (`customer_id`)
) ENGINE=InnoDB AUTO_INCREMENT=17 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
