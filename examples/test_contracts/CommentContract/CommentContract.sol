pragma solidity ^0.8.0;

contract CommentContract {
    enum Status {
        // pending comment1
        // pending comment2
        Pending,
        Shipped, /* shipped comment1
 shipped comment2
*/
        Accepted,
        Rejected, /* rejected comment */
        /* canceled comment1 */
        Canceled // canceled comment2
    }

    Status public status;

    event Log(address indexed sender, /* sender comment */ string message, // message comment
    /* priority comment1 */ uint8 priority, /* priority comment2 */ Status status);

    struct RoleData1 {
        /**
                    * MULTILINE_COMMENT::members
                */
        mapping(address => bool) members; // COMMENT::members
        //COMMENT::adminRole
        bytes32 adminRole;  /**

                * MULTILINE_COMMENT::adminRole
            */
    }

    struct RoleData2 {
        /**
                    *
           MULTILINE_COMMENT::members     */
        mapping(address => bool) members; // COMMENT::members
        bytes32 adminRole;
        // COMMENT::adminRole
    }

    struct RoleData3 {
        // COMMENT::members
        mapping(address => bool) members;
        bytes32 adminRole; //                 COMMENT::adminRole
    }

    struct RoleData4 {
        mapping(address => bool) members;  /**
                * MULTILINE_COMMENT::members
            */
        bytes32 adminRole; /**
                * MULTILINE_COMMENT::adminRole
            */
    }

    function _doSafeBatchTransferAcceptanceCheck(
        address operator,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory amounts,
        bytes memory data
    ) private {
        ids = 5;
        if (ids < 4) {
            ids = 5;
        } /*
COMMENT1
COMMENT2

COMMENT3
*/
        else if (ids == 0){
            ids = 4;
        } // COMMENT4
        else {
            ids = 0;
        }

        if (to.isContract()) {
            try IERC1155Receiver(to).onERC1155BatchReceived(operator, from, ids, amounts, data) returns (
                bytes4 response
            ) {
                if (response != IERC1155Receiver.onERC1155BatchReceived.selector) {
                    revert("ERC1155: ERC1155Receiver rejected tokens");
                }
            } // COMMENT5
            catch Error(string memory reason) {
                revert(reason);
            } /*
COMMENT6

*/
            catch {
                revert("ERC1155: transfer to non-ERC1155Receiver implementer");
            }
        }

    }
}
