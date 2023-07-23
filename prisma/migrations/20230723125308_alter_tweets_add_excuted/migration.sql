/*
  Warnings:

  - Added the required column `excuted` to the `Tweets` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `Tweets` ADD COLUMN `excuted` BOOLEAN NOT NULL;
