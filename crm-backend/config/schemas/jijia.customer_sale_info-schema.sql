/*!40101 SET NAMES binary*/;
CREATE TABLE `customer_sale_info` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `customer_id` bigint DEFAULT NULL,
  `date` timestamp NULL DEFAULT NULL,
  `sale_number` int DEFAULT NULL,
  `plateform_name` varchar(20) DEFAULT NULL,
  `store_id` int DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=70 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
