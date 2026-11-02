import {defineConfig} from '@playwright/test';
export default defineConfig({
  testDir:'tests',testMatch:'*.spec.js',workers:1,timeout:30000,
  use:{baseURL:process.env.PARLIAMENT_URL??'http://127.0.0.1:8768/TIGRIS/',headless:true,trace:'retain-on-failure',screenshot:'only-on-failure'},
  webServer:process.env.PARLIAMENT_URL?undefined:{command:'python -m http.server 8768 --directory ../../target/parliament-prototype --bind 127.0.0.1',url:'http://127.0.0.1:8768/TIGRIS/',reuseExistingServer:false},
});
