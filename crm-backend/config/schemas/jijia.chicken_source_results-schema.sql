/*!40101 SET NAMES binary*/;
CREATE TABLE `chicken_source_results` (
  `id` bigint NOT NULL,
  `create_date` timestamp NULL DEFAULT NULL,
  `order_id` bigint DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
