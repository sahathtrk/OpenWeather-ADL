# OpenWeatherMap API Automation Test (Katalon Studio)

## This project covers automation tests for:
1. 5-day weather forecast of Jakarta Selatan
2. Current air pollution of Jakarta Selatan
   
## All tests validate:
✅ Response body contents  ✅ JSON schema compliance  ✅ HTTP status codes and headers

## Requirements
- Katalon Studio installed
- A valid OpenWeatherMap API Key
- Git installed (to clone this repo)

## Project Structure

```
OpenWeather-ADL
├── Object Repository/
│   ├── Get_Geocoding
│   ├── Get_5DaysForecast
│   └── Get_CurrentPollution
├── Test Cases/
│   ├── TC_Get5DaysForecast
│   └── TC_GetCurrentAirPollution
├── Test Suites/
│   └── TS_WeatherAPI
├── Reports/
│   └── (Generated reports after running tests)
├── Profiles/
│   └── default.profile
├── README.md
└── .gitignore
```

## ▶ How to Clone and Run
1. Clone the GitHub Repository
```bash
git clone https://github.com/sahathtrk/OpenWeather-ADL.git
```
2. Open the Project in Katalon Studio
- Launch Katalon Studio
- Click File > Open Project
- Select the cloned folder
- Set Your API Key
- Open Profiles/Default
- Update the following Global Variable:
  - API_KEY
- Run the Test Suite
- Open Test Suites/OpenWeatherAPIsSuite
- Click Run (▶️) to execute all scenarios

## How to Generate Report
  After running the Test Suite:
  1. Katalon will **automatically generate a report** under:
  ```
  Reports/TS_WeatherAPI/[timestamp]/
  ```
  2. Inside the report folder, you will find:
     - `JUnit_Report.xml`
     - `HTML_Report.html`
     - `Log Files`
  3. To view a report:
     - Open the `HTML_Report.html` file in your browser.
  You can also export reports in **PDF** or **CSV** format by configuring **Project > Settings > Report**.
