/*
  Warnings:

  - Added the required column `status` to the `Subscriptions` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `Subscriptions` ADD COLUMN `status` VARCHAR(191) NOT NULL;

-- CreateTable
CREATE TABLE `SubscriptionTokens` (
    `subscription_token` VARCHAR(191) NOT NULL,
    `subscriber_id` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`subscription_token`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `SubscriptionTokens` ADD CONSTRAINT `SubscriptionTokens_subscriber_id_fkey` FOREIGN KEY (`subscriber_id`) REFERENCES `Subscriptions`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
