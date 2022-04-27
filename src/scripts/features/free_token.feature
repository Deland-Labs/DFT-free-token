@freeToken
Feature: FreeToken

  Background:
    Given Reinstall dft canisters
      | key            | name     | symbol   | decimals | total_supply | owner    |
      | token_WICP     | W ICP    | WICP     | 0        | 10^9         | dft_main |
      | token_WUSD     | W USD    | WUSD     | 0        | 10^9         | dft_main |
      | token_mintable | mintable | mintable | 0        | 10^9         | dft_main |
    And Reinstall freeToken canisters
      | user     |
      | dft_main |


  @dev
  Scenario: FreeToken
    When mintable "token_mintable" add minter "free_token"
    Then Users receive tokens for free
      | user      |
      | dft_main  |
      | dft_user1 |
      | dft_user2 |
      | dft_main  |
      | dft_main  |
      | dft_main  |
