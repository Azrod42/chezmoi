return {
  url = 'https://codeberg.org/andyg/leap.nvim',
  config = function()
    local leap = require 'leap'
    -- leap.add_default_mappings()
    leap.opts.case_sensitive = true

    vim.keymap.set({ 'n', 'x', 'o', 'v' }, 'm', '<Plug>(leap-forward)')
    vim.keymap.set({ 'n', 'x', 'o', 'v' }, 'M', '<Plug>(leap-backward)')
    vim.keymap.set({ 'n', 'x', 'o', 'v' }, 'ms', '<Plug>(leap-from-window)')

    vim.api.nvim_set_hl(0, 'LeapBackdrop', { link = 'Comment' }) -- or some grey
    vim.api.nvim_set_hl(0, 'LeapMatch', {
      fg = 'white',
      bold = true,
      nocombine = true,
    })

    vim.api.nvim_set_hl(0, 'LeapLabelPrimary', {
      fg = 'red',
      bold = true,
      nocombine = true,
    })
    vim.api.nvim_set_hl(0, 'LeapLabelSecondary', {
      fg = 'blue',
      bold = true,
      nocombine = true,
    })
    require('leap').opts.highlight_unlabeled_phase_one_targets = true
  end,
}
