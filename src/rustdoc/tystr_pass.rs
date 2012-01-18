#[doc =
  "Pulls type information out of the AST and attaches it to the document"];

import rustc::syntax::ast;
import rustc::syntax::print::pprust;
import rustc::middle::ast_map;

export mk_pass;

fn mk_pass() -> pass {
    run
}

fn run(
    srv: astsrv::srv,
    doc: doc::cratedoc
) -> doc::cratedoc {
    let fold = fold::fold({
        fold_fn: fn~(
            f: fold::fold<astsrv::srv>,
            d: doc::fndoc
        ) -> doc::fndoc {
            fold_fn(f, d)
        }
        with *fold::default_seq_fold(srv)
    });
    fold.fold_crate(fold, doc)
}

fn fold_fn(
    fold: fold::fold<astsrv::srv>,
    doc: doc::fndoc
) -> doc::fndoc {

    let srv = fold.ctxt;

    fn add_ret_ty(
        doc: option<doc::retdoc>,
        tystr: str
    ) -> option<doc::retdoc> {
        alt doc {
          some(doc) {
            fail "unimplemented";
          }
          none. {
            some({
                desc: none,
                ty: some(tystr)
            })
          }
        }
    }

    let retty = astsrv::exec(srv) {|ctxt|
        alt ctxt.map.get(doc.id) {
          ast_map::node_item(@{
            node: ast::item_fn(decl, _, _), _
          }) {
            pprust::ty_to_str(decl.output)
          }
        }
    };

    ~{
        return: add_ret_ty(doc.return, retty)
        with *doc
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_add_fn_ret_types() {
        let source = "fn a() -> int { }";
        let srv = astsrv::mk_srv_from_str(source);
        let doc = extract::from_srv(srv, "");
        let doc = run(srv, doc);
        assert option::get(doc.topmod.fns[0].return).ty == some("int");
    }
}