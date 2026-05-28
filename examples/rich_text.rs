use sge::prelude::*;

#[main("Rich text")]
fn main() -> anyhow::Result<()> {
    let mut controller = PanningCameraController::new();

    let rich_text = rich_text(
            r#"<font size=50>This text is <font color=red3>red</font>, and this text is <font color=blue3>blue</font>
    <font color=#abc>You <font size=30>may make your</font> text any</font> <font color="rgb 1.0 1.0 1.0">color <hl color=slate9>you want</hl></font>
    <font bold color="oklch 0.7 0.1184 119">Check the docume</font><font italic color=blue2>ntation for</font> <b>rich_text()</b> for more.
    <i>Lorem <ol color=green5>ipsum dolor</ol> <ul>sit amet consectetur</ul> adipiscing elit.</i>
    <ul color="red5">You <st>can</st> nest <font color=red5 size=70 bold>styles</font><noul> inside </noul>of    eachother</ul></font>
    "#,
        ).unwrap();

    rich_text.print_to_stdout();

    loop {
        controller.update();

        draw_logs();
        rich_text.draw_world(Area::new(Vec2::ZERO, Vec2::INFINITY), 1.1);

        if should_quit() {
            break;
        }

        next_frame().await;
    }

    Ok(())
}
