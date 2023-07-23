-- CreateTable
CREATE TABLE `Tweets` (
    `id` VARCHAR(191) NOT NULL,
    `user_id` VARCHAR(191) NOT NULL,
    `text` TEXT NOT NULL,
    `command` TEXT NOT NULL,
    `tweet_url` TINYTEXT NOT NULL,
    `command_type` ENUM('NFT', 'SPACE') NOT NULL,
    `date` DATETIME(3) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
