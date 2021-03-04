Feature: Newsletter sign-up service: Unhealthy status alerts
  In order to restore the newsletter sign-up service
  As a blog administrator,
  I want to be alerted when the sign-up service is unhealthy

  Background:
    Given the sign-up service is monitored

  Scenario: The sign-up service is unhealthy
    When the sign-up service is unhealthy
    Then the administrator receives one alert

  Scenario: The sign-up service is healthy
    When the sign-up service is healthy
    Then the administrator receives no alert
