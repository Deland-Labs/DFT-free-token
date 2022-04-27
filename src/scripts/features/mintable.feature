@mintable
Feature: Mintable

  Background:
    Given Reinstall dft canisters
      | key            | name     | symbol   | decimals | total_supply | owner    |
      | token_WICP     | W ICP    | WICP     | 0        | 10^9         | dft_main |
      | token_WUSD     | W USD    | WUSD     | 0        | 10^9         | dft_main |
      | token_mintable | mintable | mintable | 0        | 10^9         | dft_main |
    And Reinstall ex3 canisters
    And transfer token from "dft_main" to these users
      | token      | user      | amount |
      | token_WICP | dft_user1 | 10^8   |
      | token_WUSD | dft_user1 | 10^8   |
      | token_WICP | dft_user2 | 10^8   |
      | token_WUSD | dft_user2 | 10^8   |
    And Fusion init
      | amount_token | amount_decimals | volume_token | volume_decimals |
      | token_WUSD   | 0               | token_WICP   | 0               |
    And approve tokens from owner to canister in table
      | token      | owner     | canister | amount |
      | token_WICP | dft_user1 | fusion   | 10^8   |
      | token_WUSD | dft_user1 | fusion   | 10^8   |
      | token_WICP | dft_user2 | fusion   | 10^8   |
      | token_WUSD | dft_user2 | fusion   | 10^8   |

  Scenario: Owner mint to users
    When Owner "dft_main" mint to users
      | user      | amount |
      | dft_user1 | 998    |
      | dft_user2 | 1000   |
      | dft_user1 | 1000   |
      | dft_user2 | 998    |
    Then Check "token_mintable" mintable translation history
      | user      | amount |
      | dft_user1 | 998    |
      | dft_user2 | 1000   |
      | dft_user1 | 1000   |
      | dft_user2 | 998    |


  Scenario:
