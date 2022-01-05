pub struct ProposalView;
pub mod proposal_view {
    pub const OPERATION_NAME: &str = "ProposalView";
    pub const QUERY : & str = "query ProposalView {\n  proposalItems {\n    id\n    index\n    target\n    value\n    data\n    predecessor\n    delay\n    timestamp\n    status\n  }\n}\n" ;
    use serde::{Deserialize, Serialize};
    use std::fmt::{Display, Formatter, Result};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type BigInt = String;
    type Bytes = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ResponseData {
        #[serde(rename = "proposalItems")]
        pub proposal_items: Vec<ProposalViewProposalItems>,
    }
    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct ProposalViewProposalItems {
        pub id: ID,
        pub index: BigInt,
        pub target: Bytes,
        pub value: BigInt,
        pub data: Bytes,
        pub predecessor: Bytes,
        pub delay: BigInt,
        pub timestamp: BigInt,
        pub status: Int,
    }

    impl Display for ProposalViewProposalItems {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(
            f,
            "id: {}\nindex: {}\ntarget: {:?}\nvalue: {}\ndata: {}\npredecessor: {}\ntimestamp: {}\nstatus: {:?}",
            self.id,
            self.index,
            self.target,
            self.value,
            self.data,
            self.predecessor,
			self.timestamp,
            self.status
        )
        }
    }
}
impl graphql_client::GraphQLQuery for ProposalView {
    type Variables = proposal_view::Variables;
    type ResponseData = proposal_view::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: proposal_view::QUERY,
            operation_name: proposal_view::OPERATION_NAME,
        }
    }
}
