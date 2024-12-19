use ethers::contract::abigen;

abigen!(
    VRFContract,
    r#"[
        struct RandomnessRequest { address requester; uint256 num_values; bool fulfilled; uint256 feePaid; uint256 deadline; uint8 priority;}
        event RandomnessRequested(uint256 indexed requestId,  address indexed requester, uint256 numValues, uint256 fee, uint256 deadline,uint8 priority)
        function requestRandomness(uint256 words,uint256 fee,uint256 deadline,uint8 priority) external returns (uint256)
        function fulfillRandomness(uint256 requestId,bytes32[] memory outputs,bytes memory proof, uint256 randomNumber) external returns (bool)
        function getRequest(uint256 requestId) external view returns (RandomnessRequest memory)
        function getPendingRequests() external view returns (uint256[] memory)
    ]"#
);
