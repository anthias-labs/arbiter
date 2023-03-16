pub use weth9::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod weth9 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"name\":\"guy\",\"type\":\"address\",\"components\":[]},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"name\":\"src\",\"type\":\"address\",\"components\":[]},{\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdraw\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"name\":\"dst\",\"type\":\"address\",\"components\":[]},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[]},{\"inputs\":[{\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[{\"name\":\"src\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"guy\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"name\":\"src\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"dst\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"name\":\"dst\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"name\":\"src\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"name\":\"wad\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Withdrawal\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static WETH9_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    pub static WRITER_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> =
    ::ethers::contract::Lazy::new(|| {
        "0x60c0604052600d60808190527f577261707065642045746865720000000000000000000000000000000000000060a090815261003e91600091906100a3565b506040805180820190915260048082527f57455448000000000000000000000000000000000000000000000000000000006020909201918252610083916001916100a3565b506002805460ff1916601217905534801561009d57600080fd5b5061013e565b828054600181600116156101000203166002900490600052602060002090601f016020900481019282601f106100e457805160ff1916838001178555610111565b82800160010185558215610111579182015b828111156101115782518255916020019190600101906100f6565b5061011d929150610121565b5090565b61013b91905b8082111561011d5760008155600101610127565b90565b6106568061014d6000396000f3006080604052600436106100925760003560e01c63ffffffff16806306fdde031461009c578063095ea7b31461012657806318160ddd1461015e57806323b872dd146101855780632e1a7d4d146101af578063313ce567146101c757806370a08231146101f257806395d89b4114610213578063a9059cbb14610228578063d0e30db014610092578063dd62ed3e1461024c575b61009a610273565b005b3480156100a857600080fd5b506100b16102c2565b6040805160208082528351818301528351919283929083019185019080838360005b838110156100eb5781810151838201526020016100d3565b50505050905090810190601f1680156101185780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b34801561013257600080fd5b5061014a600160a060020a0360043516602435610350565b604080519115158252519081900360200190f35b34801561016a57600080fd5b506101736103b6565b60408051918252519081900360200190f35b34801561019157600080fd5b5061014a600160a060020a03600435811690602435166044356103bb565b3480156101bb57600080fd5b5061009a6004356104ef565b3480156101d357600080fd5b506101dc610584565b6040805160ff9092168252519081900360200190f35b3480156101fe57600080fd5b50610173600160a060020a036004351661058d565b34801561021f57600080fd5b506100b161059f565b34801561023457600080fd5b5061014a600160a060020a03600435166024356105f9565b34801561025857600080fd5b50610173600160a060020a036004358116906024351661060d565b33600081815260036020908152604091829020805434908101909155825190815291517fe1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109c9281900390910190a2565b6000805460408051602060026001851615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156103485780601f1061031d57610100808354040283529160200191610348565b820191906000526020600020905b81548152906001019060200180831161032b57829003601f168201915b505050505081565b336000818152600460209081526040808320600160a060020a038716808552908352818420869055815186815291519394909390927f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925928290030190a350600192915050565b303190565b600160a060020a0383166000908152600360205260408120548211156103e057600080fd5b600160a060020a038416331480159061041e5750600160a060020a038416600090815260046020908152604080832033845290915290205460001914155b1561047e57600160a060020a038416600090815260046020908152604080832033845290915290205482111561045357600080fd5b600160a060020a03841660009081526004602090815260408083203384529091529020805483900390555b600160a060020a03808516600081815260036020908152604080832080548890039055938716808352918490208054870190558351868152935191937fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef929081900390910190a35060019392505050565b3360009081526003602052604090205481111561050b57600080fd5b33600081815260036020526040808220805485900390555183156108fc0291849190818181858888f1935050505015801561054a573d6000803e3d6000fd5b5060408051828152905133917f7fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65919081900360200190a250565b60025460ff1681565b60036020526000908152604090205481565b60018054604080516020600284861615610100026000190190941693909304601f810184900484028201840190925281815292918301828280156103485780601f1061031d57610100808354040283529160200191610348565b60006106063384846103bb565b9392505050565b6004602090815260009283526040808420909152908252902054815600a165627a7a72305820b38b6422067b6cd29b17bad296203c258ff3cf36fe979a2bb8b7400fa77efd7d0029"
        .parse()
        .expect("invalid bytecode")
    });
}
