/**
 * SPDX-License-Identifier: MIT
 *
 * Copyright (c) 2018-2020 CENTRE SECZ
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

pragma solidity ^0.8.18;

// SafeMath is no longer needed, as Solidity 0.8+ has built-in overflow checks
import { AbstractBaseFiatToken } from "./AbstractBaseFiatToken.sol";
import { Blacklistable } from "./Blacklistable.sol";
import { EIP712 } from "../util/EIP712.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
/**
 * @title FiatToken
 * @dev ERC20 Token backed by fiat reserves
 */
contract BaseFiatToken is AbstractBaseFiatToken, PausableUpgradeable, Blacklistable {
    // Solidity 0.8+ has built-in overflow checks, SafeMath is no longer needed

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        // Not calling _disableInitializers() to allow initialize to be used in tests
    }

    string public name;
    string public symbol;
    uint8 public decimals;
    string public currency;

    mapping(address => uint256) internal balances;
    mapping(address => mapping(address => uint256)) internal allowed;
    uint256 internal totalSupply_;
    mapping(address => bool) internal minters;
    mapping(address => uint256) internal minterAllowed;
    
    // Exchange rate state variables
    uint256 public exchangeRate; // Exchange rate with 6 decimals precision (1000000 = 1:1)
    uint256 public exchangeRateDecimals; // Decimals for exchange rate
    address public rateController; // Address authorized to update exchange rate
    
    // Cap state variables
    uint256 public maxSupply; // Maximum total supply cap

    event Mint(address indexed minter, address indexed to, uint256 amount);
    event Burn(address indexed burner, uint256 amount);
    event MinterConfigured(address indexed minter, uint256 minterAllowedAmount);
    event MinterRemoved(address indexed oldMinter);
    event ExchangeRateUpdated(uint256 oldRate, uint256 newRate, address indexed updater);
    event RateControllerChanged(address indexed oldController, address indexed newController);
    event MaxSupplyUpdated(uint256 oldMaxSupply, uint256 newMaxSupply, address indexed updater);

    function baseInitialize(
        string memory tokenName,
        string memory tokenSymbol,
        string memory tokenCurrency,
        uint8 tokenDecimals,
        address newBlacklister,
        address _rateController,
        uint256 _exchangeRate,
        uint256 _maxSupply
    ) internal virtual onlyInitializing {
        // Initialize Pausable contract
        __Pausable_init();
        // Initialize Blacklistable contract
        __Blacklistable_init(newBlacklister);
        
        require(
            newBlacklister != address(0),
            "FiatToken: new blacklister is the zero address"
        );
        require(
            _rateController != address(0),
            "FiatToken: new owner is the zero address"
        );
        require(
            _maxSupply > 0,
            "FiatToken: max supply must be greater than 0"
        );
        name = tokenName;
        symbol = tokenSymbol;
        currency = tokenCurrency;
        decimals = tokenDecimals;
        blacklister = newBlacklister;
        rateController = _rateController; // Initially set rate controller to owner
        // Initialize state variables needed for the upgradeable contract
        totalSupply_ = 0;
        exchangeRate = _exchangeRate; // Initialize exchange rate, represents 1:1
        exchangeRateDecimals = 6; // Exchange rate precision is 6 decimal places
        maxSupply = _maxSupply; // Set maximum supply
        
            // Add contract address to blacklist
        blacklisted[address(this)] = true;
        }

    /**
     * @dev Throws if called by any account other than a minter or owner
     */
    modifier onlyMinters() {
        require(minters[msg.sender] || msg.sender == owner(), "FiatToken: caller is not a minter or owner");
        _;
    }

    /**
     * @dev Function to mint tokens
     * @param _to The address that will receive the minted tokens.
     * @param _amount The amount of tokens to mint. Must be less than or equal
     * to the minterAllowance of the caller.
     * @return A boolean that indicates if the operation was successful.
     */
    function mint(address _to, uint256 _amount)
        external
        virtual
        whenNotPaused
        onlyMinters
        notBlacklisted(msg.sender)
        notBlacklisted(_to)
        returns (bool)
    {
        require(_to != address(0), "FiatToken: mint to the zero address");
        require(_amount > 0, "FiatToken: mint amount not greater than 0");

        // Owner can mint without restrictions, other minters need to check their allowance
        if (msg.sender != owner()) {
            uint256 mintingAllowedAmount = minterAllowed[msg.sender];
            require(
                _amount <= mintingAllowedAmount,
                "FiatToken: mint amount exceeds minterAllowance"
            );
            minterAllowed[msg.sender] = mintingAllowedAmount - _amount;
        }

        // Check if exceeding maximum supply
        require(
            totalSupply_ + _amount <= maxSupply,
            "FiatToken: mint amount would exceed max supply"
        );

        totalSupply_ = totalSupply_ + _amount;
        balances[_to] = balances[_to] + _amount;
        emit Mint(msg.sender, _to, _amount);
        emit Transfer(address(0), _to, _amount);
        return true;
    }



    /**
     * @dev Get minter allowance for an account
     * @param minter The address of the minter
     */
    function minterAllowance(address minter) external virtual view returns (uint256) {
        return minterAllowed[minter];
    }

    /**
     * @dev Checks if account is a minter
     * @param account The address to check
     */
    function isMinter(address account) external virtual view returns (bool) {
        return minters[account] || account == owner();
    }

    /**
     * @notice Amount of remaining tokens spender is allowed to transfer on
     * behalf of the token owner
     * @param owner     Token owner's address
     * @param spender   Spender's address
     * @return Allowance amount
     */
    function allowance(address owner, address spender)
        external
        virtual
        override
        view
        returns (uint256)
    {
        return allowed[owner][spender];
    }

    /**
     * @dev Get totalSupply of token
     */
    function totalSupply() external virtual override view returns (uint256) {
        return totalSupply_;
    }

    /**
     * @dev Get token balance of an account
     * @param account address The account
     */
    function balanceOf(address account)
        external
        virtual
        override
        view
        returns (uint256)
    {
        return balances[account];
    }

    /**
     * @notice Set spender's allowance over the caller's tokens to be a given
     * value.
     * @param spender   Spender's address
     * @param value     Allowance amount
     * @return True if successful
     */
    function approve(address spender, uint256 value)
        external
        virtual
        override
        whenNotPaused
        notBlacklisted(msg.sender)
        notBlacklisted(spender)
        returns (bool)
    {
        _approve(msg.sender, spender, value);
        return true;
    }

    /**
     * @dev Internal function to set allowance
     * @param owner     Token owner's address
     * @param spender   Spender's address
     * @param value     Allowance amount
     */
    function _approve(
        address owner,
        address spender,
        uint256 value
    ) internal virtual override {
        require(owner != address(0), "ERC20: approve from the zero address");
        require(spender != address(0), "ERC20: approve to the zero address");
        allowed[owner][spender] = value;
        emit Approval(owner, spender, value);
    }

    /**
     * @notice Transfer tokens by spending allowance
     * @param from  Payer's address
     * @param to    Payee's address
     * @param value Transfer amount
     * @return True if successful
     */
    function transferFrom(
        address from,
        address to,
        uint256 value
    )
        external
        virtual
        override
        whenNotPaused
        notBlacklisted(msg.sender)
        notBlacklisted(from)
        notBlacklisted(to)
        returns (bool)
    {
        require(
            value <= allowed[from][msg.sender],
            "ERC20: transfer amount exceeds allowance"
        );
        _transfer(from, to, value);
        allowed[from][msg.sender] = allowed[from][msg.sender] - value;
        return true;
    }

    /**
     * @notice Transfer tokens from the caller
     * @param to    Payee's address
     * @param value Transfer amount
     * @return True if successful
     */
    function transfer(address to, uint256 value)
        external
        virtual
        override
        whenNotPaused
        notBlacklisted(msg.sender)
        notBlacklisted(to)
        returns (bool)
    {
        _transfer(msg.sender, to, value);
        return true;
    }

    /**
     * @notice Internal function to process transfers
     * @param from  Payer's address
     * @param to    Payee's address
     * @param value Transfer amount
     */
    function _transfer(
        address from,
        address to,
        uint256 value
    ) internal virtual override {
        require(from != address(0), "ERC20: transfer from the zero address");
        require(to != address(0), "ERC20: transfer to the zero address");
        require(
            value <= balances[from],
            "ERC20: transfer amount exceeds balance"
        );

        balances[from] = balances[from] - value;
        balances[to] = balances[to] + value;
        emit Transfer(from, to, value);
    }

    /**
     * @dev Function to add/update a new minter
     * @param minter The address of the minter
     * @param minterAllowedAmount The minting amount allowed for the minter
     * @return True if the operation was successful.
     */
    function configureMinter(address minter, uint256 minterAllowedAmount)
        external
        virtual
        whenNotPaused
        onlyOwner
        returns (bool)
    {
        minters[minter] = true;
        minterAllowed[minter] = minterAllowedAmount;
        emit MinterConfigured(minter, minterAllowedAmount);
        return true;
    }

    /**
     * @dev Function to remove a minter
     * @param minter The address of the minter to remove
     * @return True if the operation was successful.
     */
    function removeMinter(address minter)
        external
        virtual
        onlyOwner
        returns (bool)
    {
        minters[minter] = false;
        minterAllowed[minter] = 0;
        emit MinterRemoved(minter);
        return true;
    }

    /**
     * @dev allows a minter to burn some of its own tokens
     * Validates that caller is a minter and that sender is not blacklisted
     * amount is less than or equal to the minter's account balance
     * @param _amount uint256 the amount of tokens to be burned
     */
    function burn(uint256 _amount)
        external
        virtual
        whenNotPaused
        onlyMinters
        notBlacklisted(msg.sender)
    {
        uint256 balance = balances[msg.sender];
        require(_amount > 0, "FiatToken: burn amount not greater than 0");
        require(balance >= _amount, "FiatToken: burn amount exceeds balance");

        totalSupply_ = totalSupply_ - _amount;
        balances[msg.sender] = balance - _amount;
        emit Burn(msg.sender, _amount);
        emit Transfer(msg.sender, address(0), _amount);
    }




    /**
     * @notice Modifier to check if caller is the rate controller
     */
    modifier onlyRateController() {
        require(
            msg.sender == rateController,
            "FiatToken: caller is not the rate controller"
        );
        _;
    }

    /**
     * @notice Updates the exchange rate for the token
     * @param _newRate The new exchange rate with exchangeRateDecimals precision
     * @return True if the operation was successful
     */
    function updateExchangeRate(uint256 _newRate)
        external
        virtual
        whenNotPaused
        onlyRateController
        returns (bool)
    {
        require(_newRate > 0, "FiatToken: exchange rate must be greater than 0");
        
        // Maximum allowed rate change in a single update (20%)
        if (exchangeRate > 0) {
            uint256 maxChange = exchangeRate / 5; // 20% of current rate
            uint256 rateChange = _newRate > exchangeRate ? 
                                _newRate - exchangeRate : 
                                exchangeRate - _newRate;
            
            require(
                rateChange <= maxChange,
                "FiatToken: exchange rate change exceeds maximum allowed (20%)"
            );
        }
        
        uint256 oldRate = exchangeRate;
        exchangeRate = _newRate;
        
        emit ExchangeRateUpdated(oldRate, _newRate, msg.sender);
        return true;
    }

    /**
     * @notice Updates the rate controller address
     * @param _newRateController The address of the new rate controller
     */
    function updateRateController(address _newRateController) external onlyOwner {
        require(
            _newRateController != address(0),
            "FiatToken: new rate controller is the zero address"
        );
        
        address oldController = rateController;
        rateController = _newRateController;
        
        emit RateControllerChanged(oldController, _newRateController);
    }

    /**
     * @notice Get the current exchange rate
     * @return Current exchange rate with exchangeRateDecimals precision
     */
    function getExchangeRate() external view returns (uint256) {
        return exchangeRate;
    }

    /**
     * @notice Updates the maximum supply cap
     * @param _newMaxSupply The new maximum supply cap
     */
    function updateMaxSupply(uint256 _newMaxSupply) external onlyOwner {
        require(
            _newMaxSupply > 0,
            "FiatToken: new max supply must be greater than 0"
        );
        require(
            _newMaxSupply >= totalSupply_,
            "FiatToken: new max supply cannot be less than current total supply"
        );
        
        uint256 oldMaxSupply = maxSupply;
        maxSupply = _newMaxSupply;
        
        emit MaxSupplyUpdated(oldMaxSupply, _newMaxSupply, msg.sender);
    }

    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}
