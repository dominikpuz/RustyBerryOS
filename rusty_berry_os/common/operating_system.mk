DU_ARGUMENTS = --block-size=1024 --apparent-size

define disk_usage_KiB
    @printf '%s KiB\n' `du $(DU_ARGUMENTS) $(1) | cut -f1`
endef