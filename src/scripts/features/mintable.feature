@mintable
Feature: Mintable

  Background:
    Given Reinstall dft canisters
      | key            | name     | symbol   | decimals | total_supply | owner         |
      | token_WICP     | W ICP    | WICP     | 0        | 10^9         | icnaming_main |
      | token_WUSD     | W USD    | WUSD     | 0        | 10^9         | icnaming_main |
      | token_mintable | mintable | mintable | 0        | 10^9         | icnaming_main |
    And transfer token from "icnaming_main" to these users
      | token      | user           | amount |
      | token_WICP | icnaming_user1 | 10^8   |
      | token_WUSD | icnaming_user1 | 10^8   |
      | token_WICP | icnaming_user2 | 10^8   |
      | token_WUSD | icnaming_user2 | 10^8   |
    And Fusion init
      | amount_token | amount_decimals | volume_token | volume_decimals |
      | token_WUSD   | 0               | token_WICP   | 0               |
    And approve tokens from owner to canister in table
      | token      | owner          | canister | amount |
      | token_WICP | icnaming_user1 | fusion   | 10^8   |
      | token_WUSD | icnaming_user1 | fusion   | 10^8   |
      | token_WICP | icnaming_user2 | fusion   | 10^8   |
      | token_WUSD | icnaming_user2 | fusion   | 10^8   |

  Scenario: Owner mint to users
    When Owner "dft_main" mint to users
      | user           | amount |
      | icnaming_user1 | 998    |
      | icnaming_user2 | 1000   |
      | icnaming_user1 | 1000   |
      | icnaming_user2 | 998    |
    Then Check "token_mintable" mintable translation history
      | user           | amount |
      | icnaming_user1 | 998    |
      | icnaming_user2 | 1000   |
      | icnaming_user1 | 1000   |
      | icnaming_user2 | 998    |


