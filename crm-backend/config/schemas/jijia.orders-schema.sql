/*!40101 SET NAMES binary*/;
CREATE TABLE `orders` (
  `order_id` bigint NOT NULL AUTO_INCREMENT,
  `customer_id` bigint DEFAULT NULL,
  `customer_name` varchar(200) DEFAULT NULL,
  `product_number` int DEFAULT NULL,
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `finish_time` timestamp NULL DEFAULT NULL,
  `update_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  `is_profit` int DEFAULT NULL,
  `location` varchar(200) DEFAULT NULL,
  `order_status` int DEFAULT NULL,
  `comments` varchar(500) DEFAULT '',
  `order_fee_status` int DEFAULT '0',
  PRIMARY KEY (`order_id`)
) ENGINE=InnoDB AUTO_INCREMENT=42 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
