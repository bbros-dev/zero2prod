Feature: Newsletter sign-up service: Unhealthy status alerts
  In order to restore the newsletter sign-up service
  As a blog administrator,
  I want to be alerted when the sign-up service is unhealthy

  Scenario: The sign-up service is unhealthy
    Given the sign-up service is monitored
    When the sign-up service is unhealthy
    Then the administrator receives one alert

  Scenario: The sign-up service is healthy
    Given the sign-up service is monitored
    When the sign-up service is healthy
    Then the administrator receives no alert
