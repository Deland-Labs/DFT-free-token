@freeToken
Feature: FreeToken

  Background:
    Given Reinstall dft canisters
      | key            | name     | symbol   | decimals | total_supply | owner         |
      | token_WICP     | W ICP    | WICP     | 0        | 10^9         | icnaming_main |
      | token_WUSD     | W USD    | WUSD     | 0        | 10^9         | icnaming_main |
      | token_mintable | mintable | mintable | 0        | 10^9         | icnaming_main |
    And Reinstall freeToken and registrar canisters
    And give free_token some quotas from "icnaming_main"
      | quota_type | len | diff |
      | LenGte     | 7   | 1000 |
      | LenGte     | 6   | 1000 |
      | LenGte     | 5   | 1000 |
      | LenGte     | 4   | 2    |
      | LenGte     | 3   | 1000 |
      | LenGte     | 2   | 1000 |
      | LenGte     | 1   | 1000 |
    And transfer token from "icnaming_main" to canister
      | token      | canister   | amount |
      | token_WICP | free_token | 10^8   |


  Scenario: FreeToken
    When mintable "token_mintable" add minter "free_token"
    When add reward token
      | code     | quota_canister | len | diff | dicp_canister | dicp_amount | mint_canister  | mint_amount | user |
      | reward_1 | registrar      | 3   | 5    | token_WICP    | 1000        | token_mintable | 1200        | main |
    Then Users receive tokens for free code "reward_1"
      | user           |
      | icnaming_main  |
      | icnaming_user1 |
      | icnaming_user2 |
      | icnaming_main  |
      | icnaming_main  |
      | icnaming_main  |
    Then Users receive tokens for free code "reward_1" should failed, message expect "Already received, can not receive again"
      | user           |
      | icnaming_user1 |
      | icnaming_user2 |

  @dev
  Scenario: FreeToken insufficient quota, should failed Reward incomplete
    When mintable "token_mintable" add minter "free_token"
    When add reward token
      | code     | quota_canister | len | diff | dicp_canister | dicp_amount | mint_canister  | mint_amount | user          |
      | reward_1 | registrar      | 4   | 5    | token_WICP    | 1000        | token_mintable | 1200        | icnaming_main |
    Then Users receive tokens for free code "reward_1"
      | user           |
      | icnaming_main  |
      | icnaming_user1 |
      | icnaming_user2 |
    Then Users receive tokens for free code "reward_1" should failed, message expect "Reward incomplete"
      | user           |
      | icnaming_user1 |
      | icnaming_user2 |
