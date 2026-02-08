return {
  {
    'tpope/vim-dadbod',
    'kristijanhusak/vim-dadbod-completion',
  },
  {
    'kristijanhusak/vim-dadbod-ui',
    dependencies = {
      { 'tpope/vim-dadbod', lazy = true },
      { 'kristijanhusak/vim-dadbod-completion', ft = { 'sql', 'mysql', 'plsql' }, lazy = true },
    },
    cmd = {
      'DBUI',
      'DBUIToggle',
      'DBUIAddConnection',
      'DBUIFindBuffer',
    },
    init = function()
      -- Your DBUI configuration
      vim.o.foldenable = true
      vim.o.foldmethod = 'manual'
      vim.o.foldlevel = 99
      vim.g.db_ui_use_nerd_fonts = 1

      -- Set custom storage location for DBUI
      vim.g.db_ui_save_location = vim.fn.expand '~/Sync/work/db_ui'
    end,
  },
}
