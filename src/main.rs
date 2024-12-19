#[macro_use]
extern crate dotenv;

mod contracts;
mod utils;
// use noir_rs::{prove::prove_honk, srs::setup_srs, utils::WitnessMap, verify::verify_honk};

use contracts::vrf::{VRFContract};
use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    abi::Address,
    providers::{Provider, StreamExt, Ws, Http},
    signers::{LocalWallet, Signer},
    prelude::*,
};
use eyre::Result;
use log::{error, info};
use std::{sync::Arc, str::FromStr};
use utils::constants::{WSS_URL, VRF_CONTRACT_ADDRESS_ETH, SIGNER_PRIVATE_KEY};

// VRF imports
use vrf::openssl::{CipherSuite, ECVRF};
use vrf::VRF;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();
    let http_provider:Provider<Http> = Provider::<Http>::connect("https://testnet-rpc.vsgofficial.com").await;
    let provider = Arc::new(http_provider);
    let wallet = LocalWallet::from_str(SIGNER_PRIVATE_KEY)?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet.with_chain_id(chain_id)));
    let vrf_contract_address: Address = "0xf4641620fD224220668dA8efd7758f7B579d01b8".parse().unwrap();
    let vrf_contract = VRFContract::new(vrf_contract_address, client.clone());
    println!("{:?}", vrf_contract);
    // get requests from the contract and check it evry 5 seconds
    while {

        println!("{:?}", "requests");
        let requests = vrf_contract.get_pending_requests().call().await?;
        println!( "{:?}", requests);
        for request in requests {
            let request_id = request;
            let request = vrf_contract.get_request(request_id).call().await?;
            let requester = request.0;
            let num_values = request.1;
            let fee = request.2;
            let deadline = request.3;
            let priority = request.4;
            println!("Request ID: {:?}", request_id);
            println!("Requester: {:?}", requester);
            println!("Num Values: {:?}", num_values);
            println!("Fee: {:?}", fee);
            println!("Deadline: {:?}", deadline);
            println!("Priority: {:?}", priority);
            
            let bytecode = "H4sIAAAAAAAA/+1dB5gdxZHut0EraSUkkIQQ8UkiiPx641uiyDnY5KyNSCiiLERYQOScETlnRBI5ieyAsY/jfJx95ny2zz58vjvOZ2POh89XvZrW1uutWYX393rL3+vvq53Zmpqa/+8wXW+6eyZjVqY/ZYx5umzlPu2a8mTr0sBAlxF0ZYKuXNBVCLpKQTdA0FUJuoGCbpCgGyzoqgXdEEE3VNCtJ+iGCbrhgm59QbeBoBsh6EYKulGCbkNBN1rQbSToxgi6jQXdJoJuU0G3maDbXNBtIeiygm6soBsn6MYLui0F3VaCbmtBt42gmyDothV02wm67QXdDoJuR0G3k6DbWdDlBJ0VdDWCrlbQ1Qm6ekHXIOgaBV1e0DUJul0E3a6CbjdBt7ug20PQ7Zno3P3IJHqTHJtoClMm2Xp9rrhkgb5ysTBmFGAsU4CxXAHGCgUYKxVgHKAAY5UCjAMVYBykAONgBRirFWAcogDjUAUY11OAcZgCjMMVYFxfAcYNFGAcoQDjSAUYRynAuKECjKMVYNxIAcYxCjBurADjJgowbqoA42YKMG6uAOMWCjBmFWAcqwDjOAUYxyvAuKUCjFspwLi1AozbKMA4QQHGbRVg3E4Bxu0VYNxBAcYdFWDcSQHGnRVgzCnAaBVgrFGAsVYBxjoFGOsVYGxQgLFRAca8AoxNCjDuogDjrgow7qYA4+4KMO6hAOOeETDyhPFdF9F3LlfG8tb7nETSTNJC0krSRtJO0kFyOslkkikkZ5BMJZlGMp1kBslMklkkZ5LMJplDMpdkHsl8kgUkC0kWkZxFspjkbJJzSM4lOY+kk+R8kgtILiRZQnIRycUkl5BcSnIZyeUkV5BcSXIVydUk15BcS3IdyfUkN5DcSHITyc0kt5DcSrKU5DaS20nuILmT5C6Su5M8uMdnip8EO8n0nBjbLOhaBF2roGsTdO2CrkPQnS7oJgu6KYLuDEE3VdBNE3TTBd0MQTdT0M0SdGcKutmCbo6gmyvo5gm6+YJugaBbKOgWCbqzBN1iQXe2oDtH0J0r6M4TdJ2C7nxBd4Ggu1DQLRF0Fwm6iwXdJYLuUkF3maC7XNBdIeiuFHRXCbqrBd01gu5aQXedoLte0N0g6G4UdDcJupsF3S2C7lZBt1TQ3Sbobhd0dwi6OwXdXYLubkF3D0nWFCbfeUxMtrW5hrq69saadltrm3M1TS35+lxdfUtD3uZtfb6+rSZfW9uer8s3NrU0NeaabF1tu+2ob6rtSDoj7itXXLKTTJzOHc05A+TcrIRzGZBzixLOwMUNtlUJZ+BiCdumhDNw8YVtV8IZuJjDdijhDFwcYk9Xwhm42MROVsIZuHjFTlHCGbgYxp6hhDNwcY2dqoQzcLGOnaaEM3Dxj52uhDNwMZGdoYQzcHGSnamE83Ag51lKOAMXT9kzlXAGLsays5VwBi7usnOUcAYuFrNzlXAGLj6z85RwBi5ms/OVcAYujrMLlHAGLrazC5VwBi7es4uUcAYuBrRnKeEMXFxoFyvhDFysaM9Wwhm4+NGeo4QzcDGlPVcJZ+DiTHueEs5ZIOdOJZzHAjmfr4TzOCDnC5RwHg/kfKESzsDFsnaJEs7Axbf2IiWcgYt57cVKOAMXB9tLlHCeAOR8qRLOwMXL9jIlnIGLoe3lSjgDF1fbK5RwBi7Wtlcq4Qxc/G2vUsIZuJjcXq2EM3Bxur1GCWfgYnd7rRLOwMXz9jolnIGL8e31SjgDF/fbG5RwBr4swN6ohDPw5QP2JiWcgS8zsDcr4Qx8OYK9RQln4MsW7K1KOANf3mCXKuEMfBmEvU0JZ+DLJeztSjgDX1Zh71DCGfjyC3unEs7Al2nYu5RwBr6cw94N5OzW95UzvjxVBHmQW9dkV25A7bvLG6zdWGx97OcvYbF+h5f3vcn2PveHL5J3Bz4LdM4oa0oVpdgbwrcycSoKrCxWpoIKvY6caxLO9l5g/t0H5BirLMrAZYHMv/sFXy251rZ629LQ1mjbm+vzra1NtdbWNDc0N7TU5DvaW+ptvj5PPluba/J0uZrmVtuea25or0zqXcb0TBlwHtyP85XjeB+ICfiBCH4fBFaGWLwfZBkM8itiRdxMHFZUg/V+kWX0kMFWfN+hO79u1lxfRoKgJzdd3mBPRCz2SUOM10PyVKRvMRJ8ONk+Ykxh1OcOhJGgM8qaUkUptmF/R0kkuKvBRYIPA/PvESDH7yiJBJH596jRGQk+arAdok+PxQT8WAS/jwMrQyzej7MMBvmNEgk+kmBFR4LIMnrCYCu+79Cd376OBEFj8l3eYGPdFjuGHOPF9TwV6VuMBJ9MtsuMKYz63IEwEnRGWVOqKMU27A+VRIKNBhcJPgnMv2VAjh8qiQSR+feU0RkJPmWwHaJPT8cE/HQEv88AK0Ms3s+wDAb5jRIJLkuwoiNBZBk9a7AV33fozm9fR4Kg2dZd3mCzmC12dnCMT2rxVKRvMRJ8LtkuN6Yw6nMHwkjQGWVNqaIU27A/UhIJ1hpcJPgcMP+WAzl+pCQSRObf80ZnJPi8wXaIPr0QE/ALEfy+CKwMsXi/yDIY5DdKJLg8wYqOBJFl9JLBVnzfoTu/fR0JgtbRdnmDrU+12HWfMT72y1ORvsVI8OVk+4oxhVGfOxBGgs4oa0oVpdiG/QMlkeDOBhcJvgzMv1eAHH+gJBJE5t+rRmck+KrBdog+vRYT8GsR/L4OrAyxeL/OMhjkN0ok+EqCFR0JIsvoDYOt+L5Dd377OhIEvSGpyxvszUMW+0Yf4Jty+iwSfDPZrjCmMOpzB8JI0BllTamiFNuwP1YSCW5vcJHgm8D8WwHk+LGSSBCZf28ZnZHgWwbbIfr0dkzAb0fw+w6wMsTi/Q7LYJDfKJHgigQrOhJEltG7BlvxfYfu/PZ1JAh6922XN9g7ZS32Xa3Ad6D2WST4XrJ935jCqM8dCCNBZ5Q1pYpSbMP+REkkuI3BRYLvAfPvfSDHT5REgsj8+8DojAQ/MNgO0advxQT8rQh+vw2sDLF4f5tlMMhvlEjw/QQrOhJEltF3DLbi+w7d+e3rSDBrcB38WIPr4Mfh8tgCv27RZ5Hgd5Pth8YURn3uQBgJOqOsKVWUYhv2D5VEguMNLhL8LjD/PgRy/KGSSBCZf98zOiPB7xlsh+jTRzEBfxTB7/eBlSEW7++zDAb5jRIJfphgRUeCyDL6gcFWfN+hO799HQmCvlfZ5Q32HUiL/b4i8LuFfRYJ/k2y/diYwqjPHQgjQWeUNaWKUmzD/lRJJLiFwUWCfwPMv4+BHD9VEgki8+9vjc5I8G8NtkP06ZOYgD+J4PfvgJUhFu+/YxkM8hslEvw4wYqOBJFl9EODrfi+Q3d++zoS3MjgOvgxBtfBb4zLYwv8In2fRYJ/n2w/NaYw6nMHwkjQGWVNqaIU27B/pCQS3MTgIsG/B+bfp0COP1ISCSLz7x+MzkjwHwy2Q/TpRzEB/yiC3x8DK0Ms3j9mGQzyGyUS/DTBio4EkWX0jwZb8X2H7vz2dSQ40uA6+FEG18FviMtjO9rE6VRA7UmMBH+SbF3QVxD1/cT0jATd/1lTqihFN2wlkeBog4sEfwLMv8+AHP9RSSSIzL9/MjojwX8y2A7Rp5/GBPzTCH7/GVgZYvH+Z5bBIL9RIsHPEqzoSBBZRj8z2IrvO3Tnt68jweEG18Gvb3Ad/Aa4PLYjTJxOBdSexEjw58n2F8YURn3uQBgJOqOsKVWUom8+SiLBEQYXCf4cmH+/AHL8TEkkiMy/fzE6I8F/MdgO0adfxgT8ywh+fwWsDLF4/4plMMhvlEjwFwlWdCSILKN/NdiK7zt057evI8EhBtfBDzW4Dn49XB7bYSZOpwJqT2Ik+Hmy/bUxhVGfOxBGgs4oa0oVpdiG/VMlkeAwg4sEPwfm36+BHH+qJBJE5t+/GZ2R4L8ZbIfo029iAv5NBL//DqwMsXj/O8tgkN8okeCvE6zoSBBZRv9hsBXfd+jOb19HggMNroMfZHAd/GBcHttqE6dTAbUnMRL8z2T7hTGFUZ87EEaCzihrShWl6If9SiLBaoOLBP8TmH9fADn+TEkkiMy//zI6I8H/MtgO0affxgT82wh+/xtYGWLx/m+WwSC/USLBLxKs6EgQWUa/M9iK7zt057evI8EKg+vgKw2ugx+Ay2NbZeJ0KqD2JEaCv0+2XxpTGPW5A2Ek6IyyplRRih6QUBIJVhlcJPh7YP59CeT4CyWRIDL//mB0RoJ/MNgO0aevYgL+KoLf/wFWhli8/4dlMMhvlEjwywQrOhJEltEfDbbi+w7d+e3rSBDiK/GWMbgOvgyXx7bcxOlUDKYsxEjwf5Pt18YURn3uQBgJOqOsKVWUYhv2L5VEguUGFwn+LzD/vgZy/KWSSBCZf38yOiPBPxlsh+jT/8UE/H8R/P4ZWBli8f4zy2CQ3yiR4NcJVnQkCC2jDLbi+w7d/RlrChM6fzMA7A3Nuab2hobGmDjLADhbWhoam9vz9TFxlgNw1rY2tHfUNtbExFkBwNlcX9fRUV/bHBNnJQBnvc2119c0dsTEOQCAs6klV9+Qz7fGxFkFwGk78rVtTc0tMXEORJR7SzuFTbbJYRtpCn8k8UfnfEIFn2bLF1+5/c/Y/qds/2O2/yHbf5/tr2D7r7D95Wx/Gdt/hO3fx/bd/drvl7H9crZfwfYr2f4Atl/F9gcm+4NoO5ikmmQIyVCS9UiGkQzPrHwi0XVd013mE42cvD5XVMqvCtrRMVDGYOOeGBjLFGAsV4CxwvR/jJWm/2McoABjlQKMA03/xzjI9H+MgxVgrFaAcYjp/xiHmv6PcT0FGIcpwDhcAcb1Tf/HuIECjCMUYBxp+j/GUab/Y9xQAcbRCjBuZPo/xjGm/2PcWAHGTRRg3NT0f4ybmf6PcXMFGLdQgDGrAONYBRjHKcA4XgHGLRVg3EoBxq0VYNxGAcYJCjBuqwDjdgowbq8A4w4KMO6oAONOCjDurABjTgFGqwBjjQKMtQow1inAWK8AY4MCjI0KMOYVYGxSgHEXBRh3VYBxNwUYd1eAcQ8FGPeMgJEnFM4yxt/7XJ/+2YBkBMlIklEkG5KMJtmIZAzJxiSbkGxKshnJ5iRbuPmZJGNJxpGMJ9mSZCuSrUm2IZlAsi3JdiTbk+xAsiPJTiQ7k7iFBW7SYw1JLUkdST1JA0kjSZ7ETZTdhWRXkt1IdifZg2RPkokke5HsTbIPyb4k+5HsT3IAyYEkB5EcTHIIyaEkh5EcTnIEyZFJRfhGxhSuWnQZMjDQjRB0IwXdKEG3oaAbLeg2EnRjBN3Ggm4TQbepoNtM0G0u6LYQdFlBN1bQjRN04wXdloJuK0G3taDbRtBNEHTbCrrtBN32gm4HQbejoNtJ0O0s6HKCzgq6GkFXK+jqBF29oGsQdI2CLi/omgTdLoJuV0G3m6DbXdDtIej2FHQTBd1egm5vQbePoNtX0O0n6PYXdAcIugMF3UGC7mBBd4igO1TQHSboDhd0Rwi6IwWdu/llTWHKJNuJyTZXXLKuk/G+1nURRnuHS3nLcZYFOIteeAbkvEEmTlCA5lwG5DxCCWfgAgE7Ugln4IIDO0oJZ+ACBruhEs7IF9WNVsIZuMDCbqSEM3DBhh2jhDNwAYjdWAln5CvIN1HCGbhAxW6qhDNwwYvdTAln4AIau7kSzsiPS22hhDNwgY/NKuE8HMh5rBLOwAVIdpwSzsjPBo9Xwhm4QMpuqYQzcMGV3UoJZ+ACLru1Es7ABWF2GyWcgQvM7AQlnIEL1uy2SjgDF8DZ7ZRwBi6os9sr4QxcoGd3UMIZuODP7qiEM3ABod1JCWfggkS7sxLOwAWONqeEcxbI2SrhPBbIuUYJ53FAzrVKOI8Hcq5Twhm44NTWK+EMXMBqG5RwBi6ItY1KOAMX2Nq8Es4TgJyblHAGLgC2uyjhDFxQbHdVwhm4QNnupoQzcMGz3V0JZ+ACaruHEs7ABdl2TyWcgQu87UQlnIELxu1eSjgDF6DbvZVwBi5ot/so4QxcIG/3VcIZuODe7qeEM3ABv91fCWfgCwHsAUo4A18wYA9Uwhn4wgJ7kBLOwBcg2IOVcAa+UMEeooQz8AUN9lAlnIEvfLCHKeEMfIGEPVwJZ+ALKewRSjgDX3BhjwRyrky4lif++PrdMGGuWfr4V3/HWPr4FwZj6eNfGIylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMI5VgHGcAoylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMJY+/oXBWPr4FwZj6eNfGIylj39hMJY+/oXBqOXjX/F81+UyLG+9z2/SP0eRHE1yDMmxJMeRHE9yAsmJJCeRnExyCsmpJKeRTCJpJmkhaSVpI2kn6SA5nWQyyRSSM0imkkwjmU4yg2QmySySM0lmk8whmUsyj2Q+yQKShSSLSM4iWUxyNsk5JOeSnEfSSXI+yQUkF5IsIbmI5GKSS0guJbmM5HKSK0iuJLmK5GqSa0iuJbkuYwo/suOMww/vXCHorhR0Vwm6qwXdNYLuWkF3XaLjCd0oXOF7X8gP/qBxHpWJ03jROI9WgvMYJTiPVYLzOCU4j1eC8wQlOE9UgvMkJThPVoLzFCU4T1WC8zQlOCcpwdmsBGeLEpytSnC2KcHZrgRnhxKcpyvBOVkJzilKcJ6hBOdUJTinKcE5XQnOGUpwzlSCc5YSnGcqwTlbCc45SnDOVYJznhKc85XgXKAE50IlOBcpwXmWEpyLleA8WwnOc5TgPFcJzvOU4OxUgvN8JTgvUILzQiU4lyjBeZESnBcrwXmJEpyXKsF5WSScFcXitIX/Xl4czgJvVxTLmXm7EjCvyr/U8V/7a1nkCllfhcCZeLsaw7nL2zXAsvi8f5dFzrO+FofTXgf0BazL9nPU3MX2lS9RdPMuy0zPBCubyHU7V0zqv3UmxmTrVfcsXt7XJ53oDW6bMd0Tcd2BzwKdM8qaUkUp9ob6Gw031KBCryPnmoSzvR6YfzcAG22ssigDlwUy/24UfLXkWtvqbUtDW6Ntb67Pt7Y21Vpb09zQ3NBSk+9ob6m3+fo8+WxtrsnT5WqaW217rrmhvTKpdxnTM6ED9BuBZcXx3pSJCNg5R/u9GVgZYvG+meUwyK+IFXEzcVhRDdb7RZbRLeCK7zt053csbfsyErw8g+vgr8jgOvgrgZ3KVZk4nQqoPYmR4K1Jw1oaRoK3CpHg0piRYALyKmD0cSuwMS5lvmra6ttzTfmWfE1Lc21jS11tS1NTM/ltsDbf0VaTa6ur6ai3DQ2tTe1NHba2o6W+vbmhvrmpoa2r92yL9ZgDHX0g8+82pdHHbZGij9szEQHfHiH6uKOfRx+O9x1Koo+lCVZ09IEsozsjRR93JtEHT+j8vQuAPZ9sY+K8G5zHIxOMdyUdtdu/ux/vL2X7N7D9QWx/MNuvZvtD2P5Qtr8e2x/G9ocn+/fQ9l6S+0juJ3mA5EGSh0gezvSMhtH3qYeBZe6Dr0fI56Mkj5E8nlkJmrc5dzxcc/+ooHtM0D2e6bk2Hx30PQy8bz0C8LVynX/OPgrE9R+ZOEFfeVAWxXB2ZfEYJP9WluvjOM4WmX+x2/hDEdr4E+TzSZJlJE8JbfwJoe0+KeiWCbqn+qCNPwRsS08A2/iTQFxfKGnjy4Bt/ClgG/9CURt/MEIbf5p8PkPyLMlzQht/Wmi7zwi6ZwXdc33Qxh8EtqWngW38GSCu3ypp488C2/hzwDb+W0Vt/IEIbXw5+Xye5AWSF4U2vlxou88LuhcE3Yt90MYfALal5cA2/jwQ1++UtPEXgG38RWAb/52iNn5/hDb+Evl8meQVkleFNv6S0HZfFnSvCLpX+6CN3w9sSy8B2/jLQFxfKmnjrwDb+KvANv6lojZ+X4Q2/hr5fJ3kDZI3hTb+mtB2Xxd0bwi6N/ugjd8HbEuvAdv460BcXylp428A2/ibwDb+laI2fm+ENr6CfL5F8jbJO0IbXyG03bcE3duC7p0+aOP3AtvSCmAbfwuI649K2vjbwDb+DrCN/1FRG78nQht/l3y+R/I+yQdCG39XaLvvCbr3Bd0HfdDG7wG2pXeBbfw9IK6vlbTx94Ft/ANgG4+Vf+gJZxMNrs58oITzXkDO7yvhvDeQ83tKOO9j4tync0WmmJz3BXJ+Rwnn/YCc31bCeX8g57eUcD4AyHmFEs4HAjm/qYTzQUDObyjhfDCQ8+tKOB8C5PyaEs6HAjm/qoTzYUDOryjhfDiQ88tKOB8B5PySEs5HAjm/qITzN4CcX1DC+ZtAzs8r4XwUkPNyJZyPBnJ+TgnnY4Ccn1XC+Vgg52eUcD4OyPlpJZyPB3J+SgnnE4CclynhfCKQ85NKOJ8E5PyEEs4nAzk/roTzKUDOjynhfCqQ86NKOJ8G5PwIkLObV+HnUGTZdizJOJLxJFuSbEWyNck2JBNItiXZjmR7kh1IdiTZiWRnh4nEfdS7xnElqSOpJ2kgaSTJkzSR7EKyK8luJLuT7EGyJ8PhUiYpi/IEp3tZyQCSKrNyLsggksEk1SRDSIaSrEcyjGQ4yfokG5CMMCvX848i2ZBkNMlGJGNINibZhGRTks1INifZguGYSOLGlfcmceOObhzOjUu5cRo3buGe47vn2u45r3vu6Z4Duudi7jmRe27iniO439Xud6b73eV+h7i43MWpLm5zcYzr110/5+777j7o7guunZyW4Pd5MTHZv4jl0abJtnnu3Pbps+Zm587MNre1ZRdMmTs5O3N+++yOaTMX8HOvZ+eO7XnunHktc2c3t85Nd/BYERdfxs4dk2z3mj27eVF2yoy29oXZmfPmZmd2ZFtmzpvRNoef+Py6nriiCLQfsHPXS7ZHzW1unUpnzsy2tbe772qZxcnMrKvXFqI/cek6QPTn3rNGEL9OrM9n1kLZT583be6UWdMW9bycd3DXOkD15963tvnjT3x0XU98vgi0r63rRT8o4qIfsXPl0uy6Bbr0WbIdlGx9l+WnzE1M/s8Vl+wg5hftP59rbRhkChMYf+2gxGdFFP91Nd5/ZRz8uarEzz6d3f45F3/d8sAuPCfDbPZlNvum2OzHbPZLsdmf2eyfYnMAszkgxeZAZnNgis1BzOagFJuDmc3BKTaHMJtDUmwOZTaHptgcxmwOS7E5nNkcnmJzBLM5IsXmSGZzZIrNN5jNN1JsvslsvplicxSzOSrF5mhmc3SKzTHM5pgUm2OZzbEpNscxm+NSbI5nNsen2JzAbE5IsTmR2ZyYYnMSszkpxeZkZnNyis0pzOaUFJtTmc2pKTanMZvTUmwmMZtJKTbNzKY5xaaF2bSk2LQym9YUmzZm05Zi085s2lNsOphNR4rN6czm9BSbycxmcorNFGYzJcXmDGZzRorNVGYzNcVmGrOZlmIzndlMT7GZwWxmpNjMZDYzU2xmMZtZKTZnMpszU2xmM5vZKTZzmM2cFJu5zGZuis08ZjMvxWY+s5mfYrOA2SxIsVnIbBam2CxiNotSbM5iNmel2CxmNotTbM5mNmen2JzDbM5JsTmX2ZybYnMeszkvxaaT2XSm2JzPbM5PsbmA2VyQYnMhs7mQ2ZQzmyXMZklgM4j55PqJyf+5IlI+V1cbN16syw1NfFZ0U1zFxV97QJxr20xwPWO685wf89cfHGDF4snZTHA9jyfMH//baai36ezGkwmOVXT25OGPVbJjvnzdY6pJzC6sWxXBMY/FJV9/h5rCOu7Sks5u/1NNIV5uVx5c07BrxPxtlM/l6/uirndx7Cz07/PMpQp2rCI4tibl5dJ7zM7nnXukeFGi38t056dLVZ0x+Ha/83hg4t+/dJ5fk/MqC+zD/cpAd4bpxt3Fh9n665Qznb/msOT/AeyYP7c3XxWCr+GC/YDAV5Xgi+s8b9c+m5N994jYle27yf++TVV2n9bv75ORnjH0ep/k+RPeN6oYngwOT877Hxgn/3OZwD/nO1DIf19XBgnHvK/Byf+VzBe3H8jykNvzfX8+1y1OtsMFn2H+DxL4cB1vF/OS/WECn8rAr1QvBgh+hwvnh3nIz8ukbP11Ql14HQlz7D7H50mV6Zkn5QKesK4NSLH3/ioD+4uT7QZGTqF/Xv/KBP9S20X2USH+Kxnvy5P9MEbl+eXslq4lt3J2nNvfxHxek+IzsxqfYdtKK7+yAIO3vyHZppVf2A+FdeWWXuyqerFbXd5Gfm5tw3g2xHsH04f1XyqPCnac2z/EfN6d7A8Tzh8QHAvLl++XB3nCdcb0vA9lAnuXpFi/LDgm8atmGKsE/L31XVJ9XNP6y9sYt3882abVX1+PeH8W435bzfBJffXgAL+3f2o1+MP+25jefzOFeLg9zwOPzZcl77f9uX/JvON80vLuhWTb272ri39nty/ez7hU0RmFm3X5+22GI2wTley6aeXL7de2fMN7QDU7NijwJcUK/HphrCC1Vek+GrbVt5JtWnlJcZPPgzW9X8cs18Z8929cX6f4b1yeKthxbv8t03se+PtrF6/Obj2vwy7535MV7Prcnvf33P573pbk+8l++LyBX8/Z/aQXu0zKtsuHoKtIzvXlxH+Hx3gWMbizOx/Kg2tyPGWBfbhfGeh+HHCr7uzJmz+L8Nf0Zcufc/hze/NVKfgaLtgPDHxJzzW4jv/m+jjZHxxcEx178Trl6vuLHleACX3dyHNF6vtqrkh5FP91Nu5cFHmuCOfir1se2IXn8PP2ZTb7ptjsx2z2S7EpzRUp3A9tSnNFCvdDm9JckcL90KY0V6RwP7QpzRUp3A9tSnNFCvdDm9JckcL90Oavba5IX80DiTQvOSfNGfBcIo+RrPH45qrffAFWLJ7u8c3KAE+YP+s6D6QyOFbJjvnydc9ZTmV2Ut3ix6R5IM7HFFOIqVzglBE4RZ7rUad1rsepgd27zI7P9ViS6Pcy3fnpUlVnDL7x53pMNt24u/gwW+n5ir9mf57rcVqy7+d63Jn839tcjzW5V/HrSGPc3k4aS88E+2s7ls7P93Zx56fUNvnrrm5MbgDDHOLhz7wrg+MLky2fhxD2V+E9N7x38XJfEuRLpLFtG3teivcfjh+A8OeGCuXE8/Gc4Lr8GXe5cG44ZujtL2c+O5N9aUyW95th2YdjpRXCdXnZS/4zKXwypmd9Nqb3thfiWZO+m+OuXk0e+LKvDvxOTP7PFZW6xwKHJP7SypSPFXJ7P39ibcYCeezhUkVnFG5dY4H3MRzOf8GcKnZdztGYnmN7xvSsQ0MEe15OPv+GB/ZSWy4TfEn3zRDrgBT7cN6Jt78t2a5uLJD3Yz4P1vQ+0d/HAu81vedB7LFAPjflkWR/dWOBL/Rit7YxS0VyrvaxwOUBt7+GsUA/r6YvxgJ9nXL1/SuPK8AUI1byftH++3IssLRu3JTGAk1pLNAE+6FNaSywcD+0KY0FFu6HNqWxwML90KY0Fli4H9qUxgIL90Ob0lhg4X5oU1o3vtJnad14/1k3fhqzC+sWYt04X+8b+igPrmlMz7FEXka4ut69bjzm2HgXx85u/2FZVnQWXpsf4+XFx3d9efG14RNZnrlU1RmDU/zxQj7u3MWH2UrPYPw1+/N44aRk348Xfjf5vzReuDapNq97PXj3WsW08cX5AS9pXKxCyPdwXOwl5nNRUDaR+rJ8JiibshTM5QGeOGvScqvGoVaNHZieYzAcT7jG6QJTWBZ+rKWc+akS/Hj7auG6fLwmXGdaHVw3bSyhXLgeHwOPOjZtu8d2+XPRMH6LVKZNaxq/+esPDrDGit+kNX/S+xDWTySt/LivVWsG4w7Y5vJSkMIH6FyqYMeqGcAfBBhjBGouSQPe/lruJncd04eNulw4N2x83v425vPGZL+3wfT+sMi7OgVHGAhIi0OlgdewUqcNCIeLQ729Xxy/uoXcsepzeMNOqwNpExDuXw3+8AZvTO+NPsTD7XkeeHtflvwm6s8dZHp2JMgfLB7rqh8gRu7seN5ye//K9rWZvMEDApcqOqNw65q88XOGIyzDSnbd1ZWXS2H5DhXseTn5/Bse2POyloL46uA65cJ1OJaBgT1fXC3ZhwGQt1+TRflrck9ek8lIf4kXE/CgNO3FBK+vQR649JeajBTWZ37/CevzmgYpvd3feDn5/JNeTBD+QJMe+EgT7Xrre6QfTWHfs7pF+VL78nmwpj/O+vtkpO+vJg+qGYcBEThQqvEcfB3kD7c9bn593474gyFv118mFXkuazup6GemGzfnyO+XnHdZYL+2k5D4A7A1mYTEfQ0MfFUV4au3CU1Va+lrYC++wgdzazM56hPvz8SdoPcrds3Pk/3VTdD7Qy92mZRtlw9BV5Gcq32C3u8Dbn8NE/R+k+z3xQQ9X6f8g+guXMH10X3BIHbNCP5z/HetCbjw64Z5XyGcl0n5vyzY9mYb6rluqHDM+xwh6AYF25HMH7JueP+j4vgXy2gk2x8V8OT5PBGEwfvjv3fCFN6/V/XPAb4MHp8NsUj3cZ94XfHH/x8KQsNQA5wCAA=="
            // random public seed [u8; 32]
            let pub_seed:[0u8; 32] = rand::thread_rng().gen();
            // random private secret [u8; 32]
            let priv_secret= rand::thread_rng().gen();

            // Setup the SRS (Structured Reference String) for proving and verification.
            // Provide None to auto-download, or a path to a transcript file.
            setup_srs(String::from(BYTECODE), None).expect("Failed to set up SRS");

            let mut initial_witness = WitnessMap::new();

            // Insert the public seed bytes as witnesses.
            // Assuming pub_seed occupies witnesses [0..31]
            for (i, &byte) in pub_seed.iter().enumerate() {
                initial_witness.insert(Witness(i as u32), FieldElement::from(byte as u128));
            }

            // Insert the private secret bytes as witnesses.
            // Assuming priv_secret occupies witnesses [32..63]
            for (j, &byte) in priv_secret.iter().enumerate() {
                initial_witness.insert(Witness((32 + j) as u32), FieldElement::from(byte as u128));
            }

            // Start timing the proof generation
            let start = Instant::now();

            // Generate the proof and the verification key
            let (proof, vk) = prove_honk(String::from(BYTECODE), initial_witness)
                .expect("Proof generation failed");

            println!("Proof generation time: {:?}", start.elapsed());

            // Verify the proof
            let verdict = verify_honk(proof, vk).expect("Verification failed");
            println!("Proof verification verdict: {}", verdict);

            let pi = vrf.prove(secret_key, message)
                .map_err(|e| eyre::eyre!(format!("VRF prove error: {:?}", e)))?;
            let hash_output = vrf.proof_to_hash(&pi)
                .map_err(|e| eyre::eyre!(format!("VRF proof_to_hash error: {:?}", e)))?;

            let mut hash_bytes = [0u8; 32];
            hash_bytes.copy_from_slice(&hash_output[..32]);
            let randomness: H256 = H256(hash_bytes);
            let proof = vec![proof];
            // print connected wallet address to the contract
            println!("{:?}", client.address().to_string());
            let tx_future = vrf_contract
                .fulfill_randomness(request_id, proof, outputs_owned, random_number_owned);
            
            let tx = tx_future.send().await?;
            // check if the transact
            
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        true
        
    }
    
        {Ok::<(), Box<dyn std::error::Error>>(()).expect("Error in main")}
        Ok(())
    
}
