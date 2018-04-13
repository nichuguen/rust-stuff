var Migrations = artifacts.require("./Truite.sol");

module.exports = function(deployer) {
  deployer.deploy(Migrations);
};
