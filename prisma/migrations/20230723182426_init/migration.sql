-- CreateTable
CREATE TABLE `Tweets` (
    `id` VARCHAR(191) NOT NULL,
    `user_id` VARCHAR(191) NOT NULL,
    `text` TEXT NOT NULL,
    `command` TEXT NOT NULL,
    `tweet_url` TINYTEXT NOT NULL,
    `command_type` ENUM('NFT', 'SPACE') NOT NULL,
    `date` DATETIME(3) NOT NULL,
    `excuted` BOOLEAN NOT NULL DEFAULT false,
    `excuted_tx` VARCHAR(191) NOT NULL DEFAULT '',
    `excuted_date` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;