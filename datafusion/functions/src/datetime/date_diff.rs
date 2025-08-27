// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::{any::Any, sync::Arc};

use arrow::datatypes::{DataType, Field, FieldRef};
use datafusion_common::{
    internal_err, not_impl_err,
    types::{logical_date, logical_string},
    Result,
};
use datafusion_doc::Documentation;
use datafusion_expr::{
    Coercion, ColumnarValue, ReturnFieldArgs, ScalarUDFImpl, Signature, TypeSignature,
    TypeSignatureClass,
};
use datafusion_macros::user_doc;

#[user_doc(
    doc_section(label = "Time and Date Functions"),
    description = "Returns the difference between two dates or timestamps.",
    syntax_example = "date_diff(expression1, expression2, unit)",
    argument(
        name = "expression1",
        description = "Time expression to operate on. Can be a constant, column, or function."
    ),
    argument(
        name = "expression2",
        description = "Time expression to operate on. Can be a constant, column, or function."
    ),
    argument(
        name = "unit",
        description = r#"The unit of time to use for the difference calculation. Supported units are:

    - year
    - quarter (emits value in inclusive range [1, 4] based on which quartile of the year the date is in)
    - month
    - week (week of the year)
    - day (day of the month)
"#
    )
)]
#[derive(Debug)]
pub struct DateDiffFunc {
    signature: Signature,
    aliases: Vec<String>,
    return_field: FieldRef,
}

impl Default for DateDiffFunc {
    fn default() -> Self {
        Self::new()
    }
}

impl DateDiffFunc {
    pub fn new() -> Self {
        Self {
            signature: Signature::one_of(
                vec![TypeSignature::Coercible(vec![
                    Coercion::new_exact(TypeSignatureClass::Native(logical_date())),
                    Coercion::new_exact(TypeSignatureClass::Native(logical_date())),
                    Coercion::new_exact(TypeSignatureClass::Native(logical_string())),
                ])],
                datafusion_expr::Volatility::Immutable,
            ),
            aliases: vec![String::from("datediff")],
            return_field: Arc::new(Field::new("date_diff", DataType::Int32, true)),
        }
    }
}

impl ScalarUDFImpl for DateDiffFunc {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn name(&self) -> &str {
        "date_diff"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType> {
        internal_err!("return_field_from_args should be called instead")
    }

    fn return_field_from_args(&self, _args: ReturnFieldArgs) -> Result<FieldRef> {
        Ok(Arc::clone(&self.return_field))
    }

    fn invoke_with_args(
        &self,
        _args: datafusion_expr::ScalarFunctionArgs,
    ) -> Result<ColumnarValue> {
        not_impl_err!("date_diff isn't implemented for the execution")
    }

    fn aliases(&self) -> &[String] {
        &self.aliases
    }

    fn documentation(&self) -> Option<&Documentation> {
        self.doc()
    }
}
