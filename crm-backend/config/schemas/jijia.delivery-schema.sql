/*!40101 SET NAMES binary*/;
CREATE TABLE `delivery` (
  `delivery_id` bigint NOT NULL AUTO_INCREMENT,
  `customer_id` bigint DEFAULT NULL,
  `product_number` bigint NOT NULL,
  `delivery_provider` varchar(200) NOT NULL,
  `order_price` decimal(10,2) DEFAULT NULL,
  `gas_leak_rate` decimal(3,2) DEFAULT NULL,
  `destination` varchar(200) DEFAULT NULL,
  `create_date` timestamp NULL DEFAULT NULL,
  `finish_date` timestamp NULL DEFAULT NULL,
  `delivery_number` varchar(100) DEFAULT NULL,
  `delivery_status` int DEFAULT NULL,
  `comments` varchar(500) DEFAULT NULL,
  PRIMARY KEY (`delivery_id`)
) ENGINE=InnoDB AUTO_INCREMENT=29 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
