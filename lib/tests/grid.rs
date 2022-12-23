use lib::grid::Grid;

#[test]
fn grid_bool_3x3() -> anyhow::Result<()> {
    let g = Grid::try_from((
        vec![true, true, false, false, true, false, false, true, false],
        3,
    ))?;
    insta::assert_display_snapshot!(g);
    Ok(())
}

#[test]
fn grid_bool_1x9() -> anyhow::Result<()> {
    let g = Grid::try_from((
        vec![true, true, false, false, true, false, false, true, false],
        1,
    ))?;
    insta::assert_display_snapshot!(g);
    Ok(())
}

#[test]
fn grid_bool_9x1() -> anyhow::Result<()> {
    let g = Grid::try_from((
        vec![true, true, false, false, true, false, false, true, false],
        9,
    ))?;
    insta::assert_display_snapshot!(g);
    Ok(())
}

#[test]
fn grid_bool_5x5() -> anyhow::Result<()> {
    let g = Grid::try_from((
        vec![
            true, true, false, true, true, true, false, true, false, true, false, true, true, true,
            false, true, false, true, false, true, true, true, false, true, true,
        ],
        5,
    ))?;
    insta::assert_display_snapshot!(g);
    Ok(())
}
